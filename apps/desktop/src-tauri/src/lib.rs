use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use directories::ProjectDirs;
use keyring::Entry;
use serde::Serialize;
use tauri::{
    AppHandle, Manager, State, WindowEvent,
    async_runtime::Mutex,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};
use yomiage_core::{
    AppSettings, AudioData, CachedTtsProvider, ProviderKind, SynthesisRequest, TtsProvider,
    VoiceOption, build_provider,
};
use yomiage_discord::{BotConfig, BotService, BotStatus, validate_token};

const KEYRING_SERVICE: &str = "io.github.yhay81.yomiagekun";
const KEYRING_USER: &str = "discord-bot-token";
const INVITE_PERMISSIONS: u64 = 3_148_800;
const AUDIO_CACHE_BYTES: u64 = 128 * 1024 * 1024;

struct DesktopState {
    bot: Mutex<Option<BotService>>,
    settings_path: PathBuf,
    diagnostics_dir: PathBuf,
    log_dir: PathBuf,
    _log_guard: tracing_appender::non_blocking::WorkerGuard,
}

#[derive(Debug, Serialize)]
struct TokenInfo {
    username: String,
    invite_url: String,
}

#[derive(Debug, Serialize)]
struct EngineInfo {
    provider: ProviderKind,
    endpoint: String,
    version: String,
    voices: Vec<VoiceOption>,
}

#[derive(Debug, Serialize)]
struct DiagnosticExport {
    path: String,
}

#[derive(Debug, Serialize)]
struct DiagnosticReport {
    application: &'static str,
    version: &'static str,
    operating_system: &'static str,
    architecture: &'static str,
    settings: AppSettings,
    bot: BotStatus,
    log_directory: String,
    note: &'static str,
}

fn keyring_entry() -> Result<Entry, String> {
    Entry::new(KEYRING_SERVICE, KEYRING_USER).map_err(|error| error.to_string())
}

fn load_settings(path: &PathBuf) -> Result<AppSettings, String> {
    if !path.exists() {
        return Ok(AppSettings::default());
    }
    let source = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let settings: AppSettings = serde_json::from_str(&source).map_err(|error| error.to_string())?;
    settings.validate().map_err(|error| error.to_string())?;
    Ok(settings)
}

fn write_settings(path: &PathBuf, settings: &AppSettings) -> Result<(), String> {
    settings.validate().map_err(|error| error.to_string())?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let source = serde_json::to_string_pretty(settings).map_err(|error| error.to_string())?;
    fs::write(path, source).map_err(|error| error.to_string())
}

fn invite_url(application_id: u64) -> String {
    format!(
        "https://discord.com/oauth2/authorize?client_id={application_id}&scope=bot%20applications.commands&permissions={INVITE_PERMISSIONS}&integration_type=0"
    )
}

async fn token_info(token: &str) -> Result<TokenInfo, String> {
    let (username, application_id) =
        validate_token(token).await.map_err(|error| error.to_string())?;
    Ok(TokenInfo { username, invite_url: invite_url(application_id) })
}

fn provider_connection_error(settings: &AppSettings, _error: &dyn std::fmt::Display) -> String {
    let provider = settings.provider.display_name();
    tracing::warn!(provider, "TTS provider healthcheck failed");
    format!(
        "{provider}に接続できません。{provider}を起動し、準備完了後に「接続を確認」をもう一度押してください。接続先の設定も確認してください。"
    )
}

async fn inspect_engine(settings: AppSettings) -> Result<EngineInfo, String> {
    let provider = build_provider(&settings).map_err(|error| error.to_string())?;
    let version = provider
        .healthcheck()
        .await
        .map_err(|error| provider_connection_error(&settings, &error))?;
    let voices = provider
        .voices()
        .await
        .map_err(|error| format!("声の一覧を取得できませんでした: {error}"))?;
    Ok(EngineInfo { provider: settings.provider, endpoint: settings.endpoint, version, voices })
}

#[tauri::command]
#[allow(clippy::needless_pass_by_value)] // Tauri commands deserialize owned extractor values.
fn get_settings(state: State<'_, DesktopState>) -> Result<AppSettings, String> {
    load_settings(&state.settings_path)
}

#[tauri::command]
#[allow(clippy::needless_pass_by_value)] // Tauri commands deserialize owned extractor values.
fn save_settings(state: State<'_, DesktopState>, settings: AppSettings) -> Result<(), String> {
    write_settings(&state.settings_path, &settings)
}

#[tauri::command]
async fn save_and_validate_token(token: String) -> Result<TokenInfo, String> {
    let info = token_info(&token).await?;
    keyring_entry()?
        .set_password(&token)
        .map_err(|error| format!("OSの資格情報ストアへ保存できませんでした: {error}"))?;
    Ok(info)
}

#[tauri::command]
async fn saved_token_info() -> Result<Option<TokenInfo>, String> {
    let token = match keyring_entry()?.get_password() {
        Ok(token) => token,
        Err(keyring::Error::NoEntry) => return Ok(None),
        Err(error) => return Err(format!("保存済みトークンを取得できませんでした: {error}")),
    };
    token_info(&token).await.map(Some)
}

#[tauri::command]
async fn test_provider(settings: AppSettings) -> Result<EngineInfo, String> {
    inspect_engine(settings).await
}

#[tauri::command]
async fn detect_providers() -> Vec<EngineInfo> {
    let settings_for = |provider: ProviderKind| AppSettings {
        provider,
        endpoint: provider.default_endpoint().into(),
        ..AppSettings::default()
    };
    let (aivis, voicevox) = tokio::join!(
        inspect_engine(settings_for(ProviderKind::AivisSpeech)),
        inspect_engine(settings_for(ProviderKind::Voicevox))
    );
    [aivis, voicevox]
        .into_iter()
        .filter_map(|result| match result {
            Ok(info) => Some(info),
            Err(error) => {
                tracing::debug!(%error, "engine not detected");
                None
            }
        })
        .collect()
}

#[tauri::command]
async fn preview_voice(settings: AppSettings) -> Result<tauri::ipc::Response, String> {
    let provider = build_provider(&settings).map_err(|error| error.to_string())?;
    let AudioData { bytes, .. } = provider
        .synthesize(&SynthesisRequest {
            text: "こんにちは。読み上げくんです。この声で読み上げます。".into(),
            voice: settings.voice,
        })
        .await
        .map_err(|error| format!("声を試せませんでした: {error}"))?;
    Ok(tauri::ipc::Response::new(bytes.to_vec()))
}

#[tauri::command]
async fn export_diagnostics(state: State<'_, DesktopState>) -> Result<DiagnosticExport, String> {
    fs::create_dir_all(&state.diagnostics_dir).map_err(|error| error.to_string())?;
    let timestamp =
        SystemTime::now().duration_since(UNIX_EPOCH).map_err(|error| error.to_string())?.as_secs();
    let path = state.diagnostics_dir.join(format!("読み上げくん-診断-{timestamp}.json"));
    let settings = load_settings(&state.settings_path)?;
    let bot = match state.bot.lock().await.as_ref() {
        Some(service) => service.snapshot().await,
        None => BotStatus::default(),
    };
    let report = DiagnosticReport {
        application: "読み上げくん",
        version: env!("CARGO_PKG_VERSION"),
        operating_system: std::env::consts::OS,
        architecture: std::env::consts::ARCH,
        settings,
        bot,
        log_directory: state.log_dir.to_string_lossy().into_owned(),
        note: "Discordボットのトークンと読み上げた文章は含まれていません。",
    };
    let source = serde_json::to_string_pretty(&report).map_err(|error| error.to_string())?;
    fs::write(&path, source).map_err(|error| error.to_string())?;
    Ok(DiagnosticExport { path: path.to_string_lossy().into_owned() })
}

#[tauri::command]
async fn start_bot(state: State<'_, DesktopState>) -> Result<(), String> {
    let mut bot = state.bot.lock().await;
    if bot.is_some() {
        return Err("ボットはすでに起動しています".into());
    }
    let settings = load_settings(&state.settings_path)?;
    let token = keyring_entry()?
        .get_password()
        .map_err(|_| "Discordボットのトークンが保存されていません".to_owned())?;
    let provider: Arc<dyn TtsProvider> = Arc::new(CachedTtsProvider::new(
        build_provider(&settings).map_err(|e| e.to_string())?,
        AUDIO_CACHE_BYTES,
    ));
    provider.healthcheck().await.map_err(|error| provider_connection_error(&settings, &error))?;
    let service = BotService::start(
        BotConfig {
            token,
            voice: settings.voice,
            queue_capacity: settings.queue_capacity,
            max_characters: settings.max_characters,
        },
        provider,
    )
    .await
    .map_err(|error| error.to_string())?;
    *bot = Some(service);
    Ok(())
}

#[tauri::command]
async fn stop_bot(state: State<'_, DesktopState>) -> Result<(), String> {
    let service = state.bot.lock().await.take();
    if let Some(service) = service {
        service.stop().await;
    }
    Ok(())
}

#[tauri::command]
async fn bot_status(state: State<'_, DesktopState>) -> Result<BotStatus, String> {
    Ok(match state.bot.lock().await.as_ref() {
        Some(service) => service.snapshot().await,
        None => BotStatus::default(),
    })
}

fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

fn quit_application(app: &AppHandle) {
    let app = app.clone();
    tauri::async_runtime::spawn(async move {
        let service = app.state::<DesktopState>().bot.lock().await.take();
        if let Some(service) = service {
            service.stop().await;
        }
        app.exit(0);
    });
}

fn install_tray(app: &mut tauri::App) -> tauri::Result<()> {
    let show = MenuItem::with_id(app, "show", "読み上げくんを開く", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "終了", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &quit])?;
    let mut tray = TrayIconBuilder::new()
        .tooltip("読み上げくん")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => show_main_window(app),
            "quit" => quit_application(app),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_main_window(tray.app_handle());
            }
        });
    if let Some(icon) = app.default_window_icon() {
        tray = tray.icon(icon.clone());
    }
    tray.build(app)?;
    Ok(())
}

fn setup_logging(log_dir: &Path) -> tracing_appender::non_blocking::WorkerGuard {
    fs::create_dir_all(log_dir).expect("log directory should be writable");
    let file_appender = tracing_appender::rolling::daily(log_dir, "yomiage-kun.log");
    let (writer, guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "yomiage=info".into()),
        )
        .with_ansi(false)
        .with_target(false)
        .with_writer(writer)
        .compact()
        .init();
    guard
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
/// Starts the native desktop application and its Tauri event loop.
///
/// # Panics
///
/// Panics only when the OS does not provide an application data directory or Tauri cannot start.
pub fn run() {
    let project_dirs =
        ProjectDirs::from("io.github", "yhay81", "Yomiage-kun").expect("valid project directory");
    let settings_path = project_dirs.config_dir().join("settings.json");
    let log_dir = project_dirs.data_local_dir().join("logs");
    let diagnostics_dir = project_dirs.data_local_dir().join("diagnostics");
    let log_guard = setup_logging(&log_dir);

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _, _| show_main_window(app)))
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(DesktopState {
            bot: Mutex::new(None),
            settings_path,
            diagnostics_dir,
            log_dir,
            _log_guard: log_guard,
        })
        .setup(|app| {
            install_tray(app)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() == "main"
                && let WindowEvent::CloseRequested { api, .. } = event
            {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_settings,
            save_settings,
            save_and_validate_token,
            saved_token_info,
            test_provider,
            detect_providers,
            preview_voice,
            export_diagnostics,
            start_bot,
            stop_bot,
            bot_status,
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Yomiage-kun");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_error_is_actionable_and_hides_transport_details() {
        let message = provider_connection_error(
            &AppSettings::default(),
            &"error sending request for url (http://127.0.0.1:10101/version)",
        );

        assert!(message.contains("AivisSpeechを起動"));
        assert!(message.contains("接続を確認"));
        assert!(!message.contains("error sending request"));
    }
}
