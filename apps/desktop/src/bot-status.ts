export type BotState = "starting" | "running" | "stopping" | "stopped" | "failed";

export interface BotStatus {
  state: BotState;
  username: string | null;
  guild_count: number;
  active_sessions: number;
  last_error: string | null;
}

const requiredElement = <T extends HTMLElement>(id: string): T => {
  const value = document.getElementById(id);
  if (!value) throw new Error(`状態表示の要素が見つかりません: ${id}`);
  return value as T;
};

export const renderBotStatus = (status: BotStatus): void => {
  const labels: Record<BotState, string> = {
    failed: "エラー",
    running: "読み上げ中",
    starting: "準備中",
    stopped: "停止中",
    stopping: "停止中",
  };
  const titles: Record<BotState, string> = {
    failed: "設定を確認してください",
    running: "読み上げています",
    starting: "準備しています",
    stopped: "停止しています",
    stopping: "停止しています…",
  };
  const badge = requiredElement<HTMLDivElement>("statusBadge");
  badge.className = `status-badge ${status.state}`;
  const label = badge.querySelector("span");
  if (label) label.textContent = labels[status.state];
  requiredElement("primaryStatusTitle").textContent = titles[status.state];
  requiredElement("primaryStatusDetail").textContent =
    status.state === "running"
      ? status.active_sessions > 0
        ? `${status.active_sessions}件のボイスチャンネルを読み上げています。`
        : "Discordのボイスチャンネルで /join と入力してください。"
      : status.state === "failed"
        ? "設定を開き、表示された内容を確認してください。"
        : "準備ができたら、下のボタンから始められます。";
  requiredElement("botName").textContent = status.username ?? "—";
  requiredElement("guildCount").textContent = String(status.guild_count);
  requiredElement("sessionCount").textContent = String(status.active_sessions);
  const active = status.state !== "stopped";
  requiredElement<HTMLButtonElement>("startBot").disabled = active;
  requiredElement<HTMLButtonElement>("stopBot").disabled = !active;
};
