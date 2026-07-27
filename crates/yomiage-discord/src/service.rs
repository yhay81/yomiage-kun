use std::sync::{Arc, atomic::AtomicBool};

use anyhow::{Context as _, Result, anyhow};
use dashmap::DashMap;
use secrecy::{ExposeSecret as _, SecretString};
use serde::{Deserialize, Serialize};
use serenity::{
    Client,
    all::{GatewayIntents, Http, ShardManager},
};
use songbird::SerenityInit;
use tokio::{
    sync::RwLock,
    task::JoinHandle,
    time::{Duration, timeout},
};
use yomiage_core::{TtsProvider, VoiceSettings};

use crate::{handler::Handler, session::Sessions};

pub struct BotConfig {
    pub token: SecretString,
    pub voice: VoiceSettings,
    pub queue_capacity: usize,
    pub max_characters: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BotState {
    Starting,
    Running,
    Stopping,
    #[default]
    Stopped,
    Failed,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotStatus {
    pub state: BotState,
    pub username: Option<String>,
    pub guild_count: usize,
    pub active_sessions: usize,
    pub last_error: Option<String>,
}

pub struct BotService {
    shard_manager: Arc<ShardManager>,
    task: JoinHandle<()>,
    status: Arc<RwLock<BotStatus>>,
    sessions: Sessions,
}

impl BotService {
    /// Starts the Discord gateway and voice service in the background.
    ///
    /// # Errors
    ///
    /// Returns an error when configuration is invalid or the Discord client cannot be initialized.
    pub async fn start(config: BotConfig, provider: Arc<dyn TtsProvider>) -> Result<Self> {
        if config.token.expose_secret().trim().is_empty() {
            return Err(anyhow!("Discord Botトークンが設定されていません"));
        }
        if !(1..=256).contains(&config.queue_capacity) {
            return Err(anyhow!("キュー容量は1〜256にしてください"));
        }

        let status =
            Arc::new(RwLock::new(BotStatus { state: BotState::Starting, ..BotStatus::default() }));
        let sessions: Sessions = Arc::new(DashMap::new());
        let handler = Handler {
            sessions: Arc::clone(&sessions),
            provider,
            voice: config.voice,
            queue_capacity: config.queue_capacity,
            max_characters: config.max_characters,
            status: Arc::clone(&status),
            commands_registered: AtomicBool::new(false),
        };
        let intents = GatewayIntents::GUILDS
            | GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::GUILD_VOICE_STATES
            | GatewayIntents::MESSAGE_CONTENT;
        let mut client = Client::builder(config.token.expose_secret(), intents)
            .event_handler(handler)
            .register_songbird()
            .await
            .context("Discordクライアントを初期化できませんでした")?;
        let shard_manager = Arc::clone(&client.shard_manager);
        let task_status = Arc::clone(&status);
        let task = tokio::spawn(async move {
            if let Err(error) = client.start().await {
                tracing::error!(%error, "Discord client stopped with an error");
                let mut status = task_status.write().await;
                status.state = BotState::Failed;
                status.last_error = Some(error.to_string());
            } else {
                task_status.write().await.state = BotState::Stopped;
            }
        });
        Ok(Self { shard_manager, task, status, sessions })
    }

    pub async fn snapshot(&self) -> BotStatus {
        let mut status = self.status.read().await.clone();
        status.active_sessions = self.sessions.len();
        status
    }

    pub async fn stop(self) {
        self.status.write().await.state = BotState::Stopping;
        let sessions = self.sessions.iter().map(|entry| entry.value().clone()).collect::<Vec<_>>();
        self.sessions.clear();
        for session in sessions {
            session.stop().await;
        }
        self.shard_manager.shutdown_all().await;
        let mut task = self.task;
        if timeout(Duration::from_secs(5), &mut task).await.is_err() {
            task.abort();
        }
    }
}

/// Validates a Discord Bot token and returns its username and application ID.
///
/// # Errors
///
/// Returns an error for an empty token, a user token, or a failed Discord API request.
pub async fn validate_token(token: &str) -> Result<(String, u64)> {
    if token.trim().is_empty() {
        return Err(anyhow!("Discord Botトークンを入力してください"));
    }
    let http = Http::new(token);
    let user = http.get_current_user().await.context("Botトークンを検証できませんでした")?;
    if !user.bot {
        return Err(anyhow!("通常ユーザーのトークンは使用できません"));
    }
    let application = http
        .get_current_application_info()
        .await
        .context("Discord Application情報を取得できませんでした")?;
    Ok((user.name.clone(), application.id.get()))
}
