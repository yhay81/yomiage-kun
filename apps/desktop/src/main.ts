import { invoke } from "@tauri-apps/api/core";
import { disable, enable, isEnabled } from "@tauri-apps/plugin-autostart";
import { openUrl } from "@tauri-apps/plugin-opener";
import "./styles.css";

type ProviderKind = "aivis_speech" | "voicevox";
type BotState = "starting" | "running" | "stopping" | "stopped" | "failed";

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

interface BotStatus {
  state: BotState;
  username: string | null;
  guild_count: number;
  active_sessions: number;
  last_error: string | null;
}

interface TokenInfo {
  username: string;
  invite_url: string;
}

const app = document.querySelector<HTMLDivElement>("#app");
if (!app) throw new Error("App root is missing");

app.innerHTML = `
  <main class="shell">
    <header class="hero">
      <div class="brand-mark" aria-hidden="true">
        <span></span><span></span><span></span>
      </div>
      <div>
        <p class="eyebrow">LOCAL-FIRST DISCORD TTS</p>
        <h1>Yomiage-kun</h1>
        <p class="lead">Discordの日本語チャットを、あなたのPCから読み上げます。</p>
      </div>
      <div id="statusBadge" class="status-badge stopped">
        <i></i><span>停止中</span>
      </div>
    </header>

    <div id="notice" class="notice" role="status" aria-live="polite" hidden></div>

    <section class="grid">
      <article class="card setup-card">
        <div class="card-heading">
          <span class="step">01</span>
          <div>
            <h2>Discord Bot</h2>
            <p>トークンはOSの安全な資格情報ストアだけに保存されます。</p>
          </div>
        </div>
        <button id="openPortal" class="text-button" type="button">
          Discord Developer Portalを開く ↗
        </button>
        <label>
          Botトークン
          <div class="password-row">
            <input id="token" type="password" autocomplete="off" spellcheck="false"
              placeholder="Developer Portalから貼り付け" />
            <button id="toggleToken" class="icon-button" type="button" aria-label="トークンを表示">表示</button>
          </div>
        </label>
        <div class="button-row">
          <button id="saveToken" class="secondary" type="button">検証して保存</button>
          <button id="inviteBot" class="secondary" type="button" disabled>サーバーへ追加</button>
        </div>
        <p id="tokenState" class="hint">保存済みトークンを確認しています…</p>
      </article>

      <article class="card">
        <div class="card-heading">
          <span class="step">02</span>
          <div>
            <h2>音声エンジン</h2>
            <p>AivisSpeechまたはVOICEVOXをローカルで使用します。</p>
          </div>
        </div>
        <div class="two-columns">
          <label>
            プロバイダー
            <select id="provider">
              <option value="aivis_speech">AivisSpeech</option>
              <option value="voicevox">VOICEVOX</option>
            </select>
          </label>
          <label>
            話者ID
            <input id="speakerId" type="number" min="0" step="1" />
          </label>
        </div>
        <label>
          エンドポイント
          <input id="endpoint" type="url" spellcheck="false" />
        </label>
        <div class="range-grid">
          <label>速度 <output id="speedValue"></output><input id="speed" type="range" min="0.5" max="2" step="0.05" /></label>
          <label>抑揚 <output id="intonationValue"></output><input id="intonation" type="range" min="0" max="2" step="0.05" /></label>
          <label>音量 <output id="volumeValue"></output><input id="volume" type="range" min="0" max="2" step="0.05" /></label>
        </div>
        <div class="button-row">
          <button id="testProvider" class="secondary" type="button">接続テスト</button>
          <button id="saveSettings" class="secondary" type="button">設定を保存</button>
        </div>
      </article>

      <article class="card run-card">
        <div class="card-heading">
          <span class="step">03</span>
          <div>
            <h2>読み上げを開始</h2>
            <p>Bot起動後、Discordで <code>/join</code> を実行してください。</p>
          </div>
        </div>
        <dl class="metrics">
          <div><dt>Bot</dt><dd id="botName">—</dd></div>
          <div><dt>参加サーバー</dt><dd id="guildCount">0</dd></div>
          <div><dt>読み上げ接続</dt><dd id="sessionCount">0</dd></div>
        </dl>
        <label class="checkbox">
          <input id="autostart" type="checkbox" />
          <span>ログイン時に自動起動する</span>
        </label>
        <div class="button-row primary-actions">
          <button id="startBot" class="primary" type="button">Botを開始</button>
          <button id="stopBot" class="danger" type="button" disabled>停止</button>
        </div>
      </article>
    </section>

    <footer>
      <span>音声とDiscordトークンは、このPCの外へ保存しません。</span>
      <button id="openDocs" class="text-button" type="button">セットアップガイド ↗</button>
    </footer>
  </main>
`;

const element = <T extends HTMLElement>(id: string): T => {
  const value = document.getElementById(id);
  if (!value) throw new Error(`Missing element: ${id}`);
  return value as T;
};

const notice = element<HTMLDivElement>("notice");
const statusBadge = element<HTMLDivElement>("statusBadge");
const tokenInput = element<HTMLInputElement>("token");
const inviteButton = element<HTMLButtonElement>("inviteBot");
const startButton = element<HTMLButtonElement>("startBot");
const stopButton = element<HTMLButtonElement>("stopBot");
let inviteUrl = "";

function message(text: string, type: "success" | "error" | "info" = "info"): void {
  notice.textContent = text;
  notice.className = `notice ${type}`;
  notice.hidden = false;
}

function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

function readSettings(): AppSettings {
  return {
    provider: element<HTMLSelectElement>("provider").value as ProviderKind,
    endpoint: element<HTMLInputElement>("endpoint").value.trim(),
    voice: {
      speaker_id: Number(element<HTMLInputElement>("speakerId").value),
      speed: Number(element<HTMLInputElement>("speed").value),
      pitch: 0,
      intonation: Number(element<HTMLInputElement>("intonation").value),
      volume: Number(element<HTMLInputElement>("volume").value),
    },
    max_characters: 160,
    queue_capacity: 32,
    autostart: element<HTMLInputElement>("autostart").checked,
  };
}

function setSettings(settings: AppSettings): void {
  element<HTMLSelectElement>("provider").value = settings.provider;
  element<HTMLInputElement>("endpoint").value = settings.endpoint;
  element<HTMLInputElement>("speakerId").value = String(settings.voice.speaker_id);
  element<HTMLInputElement>("speed").value = String(settings.voice.speed);
  element<HTMLInputElement>("intonation").value = String(settings.voice.intonation);
  element<HTMLInputElement>("volume").value = String(settings.voice.volume);
  updateRangeOutputs();
}

function updateRangeOutputs(): void {
  for (const name of ["speed", "intonation", "volume"]) {
    element<HTMLOutputElement>(`${name}Value`).value =
      Number(element<HTMLInputElement>(name).value).toFixed(2);
  }
}

async function persistSettings(): Promise<AppSettings> {
  const settings = readSettings();
  await invoke("save_settings", { settings });
  const shouldAutostart = settings.autostart;
  const enabled = await isEnabled();
  if (shouldAutostart && !enabled) await enable();
  if (!shouldAutostart && enabled) await disable();
  return settings;
}

function renderStatus(status: BotStatus): void {
  const labels: Record<BotState, string> = {
    starting: "接続中",
    running: "稼働中",
    stopping: "停止中",
    stopped: "停止",
    failed: "エラー",
  };
  statusBadge.className = `status-badge ${status.state}`;
  statusBadge.querySelector("span")!.textContent = labels[status.state];
  element("botName").textContent = status.username ?? "—";
  element("guildCount").textContent = String(status.guild_count);
  element("sessionCount").textContent = String(status.active_sessions);
  const hosted = status.state !== "stopped";
  startButton.disabled = hosted;
  stopButton.disabled = !hosted;
  if (status.last_error) message(status.last_error, "error");
}

async function refreshStatus(): Promise<void> {
  try {
    const status = await invoke<BotStatus>("bot_status");
    renderStatus(status);
  } catch (error) {
    console.error(error);
  }
}

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
    element("tokenState").textContent = `${info.username} として検証済みです。`;
    message("Botトークンを安全に保存しました。", "success");
  } catch (error) {
    message(errorMessage(error), "error");
  }
});

inviteButton.addEventListener("click", () => {
  if (inviteUrl) void openUrl(inviteUrl);
});

element("provider").addEventListener("change", (event) => {
  const provider = (event.target as HTMLSelectElement).value as ProviderKind;
  element<HTMLInputElement>("endpoint").value =
    provider === "aivis_speech" ? "http://127.0.0.1:10101" : "http://127.0.0.1:50021";
});

for (const name of ["speed", "intonation", "volume"]) {
  element(name).addEventListener("input", updateRangeOutputs);
}

element("testProvider").addEventListener("click", async () => {
  try {
    const settings = await persistSettings();
    const result = await invoke<string>("test_provider", { settings });
    message(`音声エンジンに接続できました（${result}）。`, "success");
  } catch (error) {
    message(errorMessage(error), "error");
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
    message("Botを起動しています。Discordでオンラインになるまでお待ちください。", "success");
    await refreshStatus();
  } catch (error) {
    message(errorMessage(error), "error");
  }
});

stopButton.addEventListener("click", async () => {
  try {
    await invoke("stop_bot");
    message("Botを停止しました。", "info");
    await refreshStatus();
  } catch (error) {
    message(errorMessage(error), "error");
  }
});

async function initialize(): Promise<void> {
  try {
    const [settings, autostartEnabled] = await Promise.all([
      invoke<AppSettings>("get_settings"),
      isEnabled(),
    ]);
    setSettings(settings);
    element<HTMLInputElement>("autostart").checked = autostartEnabled;
    let tokenInfo: TokenInfo | null = null;
    let tokenLookupFailed = false;
    try {
      tokenInfo = await invoke<TokenInfo | null>("saved_token_info");
    } catch (error) {
      tokenLookupFailed = true;
      element("tokenState").textContent = "保存済みトークンを現在検証できません。";
      message(
        `Discordへの接続を確認してください: ${errorMessage(error)}`,
        "error",
      );
    }
    if (tokenInfo) {
      inviteUrl = tokenInfo.invite_url;
      inviteButton.disabled = false;
      element("tokenState").textContent = `${tokenInfo.username} のトークンを保存済みです。`;
    } else if (!tokenLookupFailed) {
      element("tokenState").textContent = "Botトークンはまだ保存されていません。";
    }
    if (autostartEnabled && tokenInfo) {
      try {
        await invoke("start_bot");
        message("自動起動設定によりBotを開始しました。", "success");
      } catch (error) {
        message(`Botを自動起動できませんでした: ${errorMessage(error)}`, "error");
      }
    }
    await refreshStatus();
  } catch (error) {
    message(errorMessage(error), "error");
  }
}

void initialize();
window.setInterval(() => void refreshStatus(), 2_000);
