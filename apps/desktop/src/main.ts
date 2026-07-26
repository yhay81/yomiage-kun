import { invoke } from "@tauri-apps/api/core";
import { disable, enable, isEnabled } from "@tauri-apps/plugin-autostart";
import { openUrl } from "@tauri-apps/plugin-opener";
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

const STATUS_REFRESH_MS = 2_000;
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
const inviteButton = element<HTMLButtonElement>("inviteBot");
const startButton = element<HTMLButtonElement>("startBot");
const stopButton = element<HTMLButtonElement>("stopBot");
const testProviderButton = element<HTMLButtonElement>("testProvider");
let inviteUrl = "";

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

const currentProviderName = (): string =>
  providerSelect.selectedOptions.item(0)?.textContent ?? "音声エンジン";

const readSettings = (): AppSettings => ({
  autostart: element<HTMLInputElement>("autostart").checked,
  endpoint: element<HTMLInputElement>("endpoint").value.trim(),
  max_characters: 160,
  provider: element<HTMLSelectElement>("provider").value as ProviderKind,
  queue_capacity: 32,
  voice: {
    intonation: Number(element<HTMLInputElement>("intonation").value),
    pitch: 0,
    speaker_id: Number(element<HTMLInputElement>("speakerId").value),
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

const setSettings = (settings: AppSettings): void => {
  element<HTMLSelectElement>("provider").value = settings.provider;
  element<HTMLInputElement>("endpoint").value = settings.endpoint;
  element<HTMLInputElement>("speakerId").value = String(settings.voice.speaker_id);
  element<HTMLInputElement>("speed").value = String(settings.voice.speed);
  element<HTMLInputElement>("intonation").value = String(settings.voice.intonation);
  element<HTMLInputElement>("volume").value = String(settings.voice.volume);
  updateRangeOutputs();
};

const persistSettings = async (): Promise<AppSettings> => {
  const settings = readSettings();
  await invoke("save_settings", { settings });
  const shouldAutostart = settings.autostart;
  const enabled = await isEnabled();
  if (shouldAutostart && !enabled) await enable();
  if (!shouldAutostart && enabled) await disable();
  return settings;
};

const renderStatus = (status: BotStatus): void => {
  renderBotStatus(status);
  if (status.last_error) message(status.last_error, "error");
};

const refreshStatus = async (): Promise<void> => {
  try {
    const status = await invoke<BotStatus>("bot_status");
    renderStatus(status);
  } catch (error) {
    console.error(error);
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
  openUrl("https://github.com/yhay81/yomiage-kun/blob/master/docs/discord-setup.md"),
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
    element("tokenState").textContent = `${info.username} として検証済みです。`;
    message("ボットのトークンを安全に保存しました。", "success");
  } catch (error) {
    message(errorMessage(error), "error");
  }
});

inviteButton.addEventListener("click", () => {
  if (inviteUrl) void openUrl(inviteUrl);
});

providerSelect.addEventListener("change", (event) => {
  const provider = (event.target as HTMLSelectElement).value as ProviderKind;
  element<HTMLInputElement>("endpoint").value =
    provider === "aivis_speech"
      ? "http://127.0.0.1:10101"
      : "http://127.0.0.1:50021";
  renderEngineStatus("idle", currentProviderName());
});

element("endpoint").addEventListener("input", () => {
  renderEngineStatus("idle", currentProviderName());
});

for (const name of ["speed", "intonation", "volume"]) {
  element(name).addEventListener("input", updateRangeOutputs);
}

testProviderButton.addEventListener("click", async () => {
  testProviderButton.disabled = true;
  testProviderButton.textContent = "確認中…";
  renderEngineStatus("testing", currentProviderName());
  try {
    const settings = await persistSettings();
    const result = await invoke<string>("test_provider", { settings });
    renderEngineStatus("ready", currentProviderName());
    message(`音声エンジンに接続できました（${result}）。`, "success");
  } catch (error) {
    renderEngineStatus("unavailable", currentProviderName());
    message(errorMessage(error), "error");
  } finally {
    testProviderButton.disabled = false;
    testProviderButton.textContent = "接続を確認";
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
      element("tokenState").textContent = "保存済みトークンを現在検証できません。";
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
    await refreshStatus();
  } catch (error) {
    message(errorMessage(error), "error");
  }
};

void initialize();
window.setInterval(() => void refreshStatus(), STATUS_REFRESH_MS);
