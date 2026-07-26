import { invoke } from "@tauri-apps/api/core";
import { disable, enable, isEnabled } from "@tauri-apps/plugin-autostart";
import { openPath, openUrl } from "@tauri-apps/plugin-opener";
import { relaunch } from "@tauri-apps/plugin-process";
import { check, type Update } from "@tauri-apps/plugin-updater";
import { type BotStatus, renderBotStatus } from "./bot-status";
import { renderEngineStatus } from "./engine-status";
import { installSettingsDialog } from "./settings-dialog";
import { appMarkup } from "./ui";
import { installWindowControls } from "./window-controls";

type ProviderKind = "aivis_speech" | "voicevox";

interface VoiceSettings {
  speaker_id: number;
  speed: number;
  pitch: number;
  intonation: number;
  volume: number;
}

interface AppSettings {
  provider: ProviderKind;
  endpoint: string;
  voice: VoiceSettings;
  max_characters: number;
  queue_capacity: number;
  autostart: boolean;
}

interface TokenInfo {
  username: string;
  invite_url: string;
}

interface VoiceOption {
  id: number;
  name: string;
}

interface EngineInfo {
  provider: ProviderKind;
  endpoint: string;
  version: string;
  voices: VoiceOption[];
}

interface DiagnosticExport {
  path: string;
}

const STATUS_REFRESH_MS = 2_000;
const PROVIDER_ENDPOINTS: Record<ProviderKind, string> = {
  aivis_speech: "http://127.0.0.1:10101",
  voicevox: "http://127.0.0.1:50021",
};
const PROVIDER_SITES: Record<ProviderKind, string> = {
  aivis_speech: "https://aivis-project.com/",
  voicevox: "https://voicevox.hiroshiba.jp/",
};

const app = document.querySelector<HTMLDivElement>("#app");
if (!app) throw new Error("App root is missing");
app.innerHTML = appMarkup;
installWindowControls();
installSettingsDialog();

const element = <T extends HTMLElement>(id: string): T => {
  const value = document.getElementById(id);
  if (!value) throw new Error(`Missing element: ${id}`);
  return value as T;
};

const notice = element<HTMLDivElement>("notice");
const tokenInput = element<HTMLInputElement>("token");
const providerSelect = element<HTMLSelectElement>("provider");
const voiceSelect = element<HTMLSelectElement>("speakerId");
const inviteButton = element<HTMLButtonElement>("inviteBot");
const startButton = element<HTMLButtonElement>("startBot");
const stopButton = element<HTMLButtonElement>("stopBot");
const testProviderButton = element<HTMLButtonElement>("testProvider");
const detectProviderButton = element<HTMLButtonElement>("detectProvider");
const previewVoiceButton = element<HTMLButtonElement>("previewVoice");
const updateButton = element<HTMLButtonElement>("installUpdate");
const detectedEngines = new Map<ProviderKind, EngineInfo>();
let inviteUrl = "";
let pendingUpdate: Update | null = null;

const message = (
  text: string,
  type: "success" | "error" | "info" = "info",
): void => {
  notice.textContent = text;
  notice.className = `notice ${type}`;
  notice.hidden = false;
};

const errorMessage = (error: unknown): string =>
  error instanceof Error ? error.message : String(error);

const currentProvider = (): ProviderKind =>
  providerSelect.value as ProviderKind;

const currentProviderName = (): string =>
  providerSelect.selectedOptions.item(0)?.textContent ?? "音声合成ソフト";

const readSettings = (): AppSettings => ({
  autostart: element<HTMLInputElement>("autostart").checked,
  endpoint: element<HTMLInputElement>("endpoint").value.trim(),
  max_characters: 160,
  provider: currentProvider(),
  queue_capacity: 32,
  voice: {
    intonation: Number(element<HTMLInputElement>("intonation").value),
    pitch: 0,
    speaker_id: Number(voiceSelect.value),
    speed: Number(element<HTMLInputElement>("speed").value),
    volume: Number(element<HTMLInputElement>("volume").value),
  },
});

const updateRangeOutputs = (): void => {
  for (const name of ["speed", "intonation", "volume"]) {
    element<HTMLOutputElement>(`${name}Value`).value = Number(
      element<HTMLInputElement>(name).value,
    ).toFixed(2);
  }
};

const setVoiceOptions = (
  voices: VoiceOption[],
  preferredSpeakerId: number,
): void => {
  voiceSelect.replaceChildren();
  for (const voice of voices) {
    const option = document.createElement("option");
    option.value = String(voice.id);
    option.textContent = voice.name;
    voiceSelect.append(option);
  }
  const preferred = voices.find((voice) => voice.id === preferredSpeakerId);
  if (preferred) voiceSelect.value = String(preferred.id);
  voiceSelect.disabled = voices.length === 0;
  previewVoiceButton.disabled = voices.length === 0;
  if (voices.length === 0) {
    const option = document.createElement("option");
    option.value = String(preferredSpeakerId);
    option.textContent = "接続すると声を選べます";
    voiceSelect.append(option);
  }
};

const setSettings = (settings: AppSettings): void => {
  providerSelect.value = settings.provider;
  element<HTMLInputElement>("endpoint").value = settings.endpoint;
  setVoiceOptions([], settings.voice.speaker_id);
  element<HTMLInputElement>("speed").value = String(settings.voice.speed);
  element<HTMLInputElement>("intonation").value = String(settings.voice.intonation);
  element<HTMLInputElement>("volume").value = String(settings.voice.volume);
  updateRangeOutputs();
};

const applyEngineInfo = (
  info: EngineInfo,
  preferredSpeakerId = Number(voiceSelect.value),
): void => {
  detectedEngines.set(info.provider, info);
  providerSelect.value = info.provider;
  element<HTMLInputElement>("endpoint").value = info.endpoint;
  setVoiceOptions(info.voices, preferredSpeakerId);
  renderEngineStatus("ready", currentProviderName(), `${info.voices.length}種類の声を選べます。`);
};

const persistSettings = async (): Promise<AppSettings> => {
  const settings = readSettings();
  await invoke("save_settings", { settings });
  const enabled = await isEnabled();
  if (settings.autostart && !enabled) await enable();
  if (!settings.autostart && enabled) await disable();
  return settings;
};

const renderStatus = (status: BotStatus): void => {
  renderBotStatus(status);
  if (status.last_error) message(status.last_error, "error");
};

const refreshStatus = async (): Promise<void> => {
  try {
    renderStatus(await invoke<BotStatus>("bot_status"));
  } catch (error) {
    console.error(error);
  }
};

const detectProviders = async (quiet = false): Promise<void> => {
  detectProviderButton.disabled = true;
  detectProviderButton.textContent = "探しています…";
  renderEngineStatus("testing", currentProviderName());
  try {
    const engines = await invoke<EngineInfo[]>("detect_providers");
    for (const engine of engines) detectedEngines.set(engine.provider, engine);
    const selected =
      detectedEngines.get(currentProvider()) ?? engines.at(0);
    if (!selected) {
      renderEngineStatus("unavailable", currentProviderName());
      if (!quiet) {
        message(
          "音声合成ソフトが見つかりません。AivisSpeechまたはVOICEVOXを起動してください。",
          "error",
        );
      }
      return;
    }
    applyEngineInfo(selected);
    if (!quiet) {
      message(`${currentProviderName()}を自動で見つけました。`, "success");
    }
  } catch (error) {
    renderEngineStatus("unavailable", currentProviderName());
    if (!quiet) message(errorMessage(error), "error");
  } finally {
    detectProviderButton.disabled = false;
    detectProviderButton.textContent = "自動で見つける";
  }
};

const testSelectedProvider = async (): Promise<EngineInfo> => {
  testProviderButton.disabled = true;
  testProviderButton.textContent = "確認中…";
  renderEngineStatus("testing", currentProviderName());
  try {
    const settings = await persistSettings();
    const info = await invoke<EngineInfo>("test_provider", { settings });
    applyEngineInfo(info, settings.voice.speaker_id);
    message(
      `${currentProviderName()}に接続できました（${info.version}）。`,
      "success",
    );
    return info;
  } catch (error) {
    renderEngineStatus("unavailable", currentProviderName());
    message(errorMessage(error), "error");
    throw error;
  } finally {
    testProviderButton.disabled = false;
    testProviderButton.textContent = "接続を確認";
  }
};

element("toggleToken").addEventListener("click", () => {
  const visible = tokenInput.type === "text";
  tokenInput.type = visible ? "password" : "text";
  element("toggleToken").textContent = visible ? "表示" : "隠す";
});

element("openPortal").addEventListener("click", () =>
  openUrl("https://discord.com/developers/applications"),
);
element("openDocs").addEventListener("click", () =>
  openUrl("https://yomiage.yusuke-hayashi.com/#start"),
);
element("openEngineSite").addEventListener("click", () =>
  openUrl(PROVIDER_SITES[currentProvider()]),
);

element("saveToken").addEventListener("click", async () => {
  try {
    const info = await invoke<TokenInfo>("save_and_validate_token", {
      token: tokenInput.value.trim(),
    });
    inviteUrl = info.invite_url;
    inviteButton.disabled = false;
    tokenInput.value = "";
    element("tokenSummary").textContent = `${info.username} と連携済みです。`;
    element("tokenSummary").closest(".readiness-item")?.classList.add("ready");
    element("tokenState").textContent = `${info.username} として確認できました。`;
    message("ボットのトークンを安全に保存しました。", "success");
  } catch (error) {
    message(errorMessage(error), "error");
  }
});

inviteButton.addEventListener("click", () => {
  if (inviteUrl) void openUrl(inviteUrl);
});

providerSelect.addEventListener("change", () => {
  const provider = currentProvider();
  element<HTMLInputElement>("endpoint").value = PROVIDER_ENDPOINTS[provider];
  const detected = detectedEngines.get(provider);
  if (detected) {
    applyEngineInfo(detected);
  } else {
    setVoiceOptions([], Number(voiceSelect.value));
    renderEngineStatus("idle", currentProviderName());
  }
});

element("endpoint").addEventListener("input", () => {
  setVoiceOptions([], Number(voiceSelect.value));
  renderEngineStatus("idle", currentProviderName());
});

for (const name of ["speed", "intonation", "volume"]) {
  element(name).addEventListener("input", updateRangeOutputs);
}

detectProviderButton.addEventListener("click", () => void detectProviders());
testProviderButton.addEventListener("click", () => void testSelectedProvider());

previewVoiceButton.addEventListener("click", async () => {
  previewVoiceButton.disabled = true;
  previewVoiceButton.textContent = "準備中…";
  try {
    if (!detectedEngines.has(currentProvider())) await testSelectedProvider();
    const settings = readSettings();
    const audio = await invoke<ArrayBuffer>("preview_voice", { settings });
    const url = URL.createObjectURL(new Blob([audio], { type: "audio/wav" }));
    const player = new Audio(url);
    player.addEventListener("ended", () => URL.revokeObjectURL(url), {
      once: true,
    });
    await player.play();
    message("選んだ声を再生しています。", "success");
  } catch (error) {
    message(errorMessage(error), "error");
  } finally {
    previewVoiceButton.disabled = voiceSelect.disabled;
    previewVoiceButton.textContent = "この声を試す";
  }
});

element("saveSettings").addEventListener("click", async () => {
  try {
    await persistSettings();
    message("設定を保存しました。", "success");
  } catch (error) {
    message(errorMessage(error), "error");
  }
});

element("exportDiagnostics").addEventListener("click", async () => {
  try {
    const result = await invoke<DiagnosticExport>("export_diagnostics");
    await openPath(result.path);
    message(
      "トークンや読み上げ内容を含まない診断情報を保存しました。",
      "success",
    );
  } catch (error) {
    message(errorMessage(error), "error");
  }
});

startButton.addEventListener("click", async () => {
  try {
    await persistSettings();
    await invoke("start_bot");
    message("読み上げの準備をしています。少しお待ちください。", "success");
    await refreshStatus();
  } catch (error) {
    if (errorMessage(error).includes("に接続できません")) {
      renderEngineStatus("unavailable", currentProviderName());
    }
    message(errorMessage(error), "error");
  }
});

stopButton.addEventListener("click", async () => {
  try {
    await invoke("stop_bot");
    message("読み上げを停止しました。", "info");
    await refreshStatus();
  } catch (error) {
    message(errorMessage(error), "error");
  }
});

updateButton.addEventListener("click", async () => {
  if (!pendingUpdate) return;
  updateButton.disabled = true;
  updateButton.textContent = "更新しています…";
  try {
    await pendingUpdate.downloadAndInstall();
    await relaunch();
  } catch (error) {
    updateButton.disabled = false;
    updateButton.textContent = "更新する";
    message(`更新できませんでした: ${errorMessage(error)}`, "error");
  }
});

const checkForUpdates = async (): Promise<void> => {
  try {
    pendingUpdate = await check();
    if (!pendingUpdate) return;
    updateButton.hidden = false;
    updateButton.textContent = `v${pendingUpdate.version}へ更新`;
  } catch (error) {
    console.info("更新確認を省略しました", error);
  }
};

const initialize = async (): Promise<void> => {
  try {
    const [settings, autostartEnabled] = await Promise.all([
      invoke<AppSettings>("get_settings"),
      isEnabled(),
    ]);
    setSettings(settings);
    renderEngineStatus("idle", currentProviderName());
    element<HTMLInputElement>("autostart").checked = autostartEnabled;
    let tokenInfo: TokenInfo | null = null;
    let tokenLookupFailed = false;
    try {
      tokenInfo = await invoke<TokenInfo | null>("saved_token_info");
    } catch (error) {
      tokenLookupFailed = true;
      element("tokenState").textContent = "保存済みトークンを現在確認できません。";
      element("tokenSummary").textContent = "現在確認できません。";
      message(`Discordへの接続を確認してください: ${errorMessage(error)}`, "error");
    }
    if (tokenInfo) {
      inviteUrl = tokenInfo.invite_url;
      inviteButton.disabled = false;
      element("tokenSummary").textContent = `${tokenInfo.username} と連携済みです。`;
      element("tokenSummary").closest(".readiness-item")?.classList.add("ready");
      element("tokenState").textContent =
        `${tokenInfo.username} のトークンを保存済みです。`;
    } else if (!tokenLookupFailed) {
      element("tokenState").textContent =
        "ボットのトークンはまだ保存されていません。";
      element("tokenSummary").textContent = "設定が必要です。";
    }
    await detectProviders(true);
    if (autostartEnabled && tokenInfo) {
      try {
        await invoke("start_bot");
        message("自動起動の設定により読み上げを開始しました。", "success");
      } catch (error) {
        message(
          `読み上げを自動で開始できませんでした: ${errorMessage(error)}`,
          "error",
        );
      }
    }
    await Promise.all([refreshStatus(), checkForUpdates()]);
  } catch (error) {
    message(errorMessage(error), "error");
  }
};

void initialize();
window.setInterval(() => void refreshStatus(), STATUS_REFRESH_MS);
