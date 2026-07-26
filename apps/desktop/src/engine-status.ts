export type EngineState = "idle" | "testing" | "ready" | "unavailable";

const requiredElement = <T extends HTMLElement>(id: string): T => {
  const value = document.getElementById(id);
  if (!value) throw new Error(`Missing engine status element: ${id}`);
  return value as T;
};

export const renderEngineStatus = (
  state: EngineState,
  providerName: string,
  readyDetail = "読み上げを開始できます。",
): void => {
  const status = requiredElement<HTMLDivElement>("engineStatus");
  const title = requiredElement<HTMLElement>("engineStatusTitle");
  const detail = requiredElement<HTMLParagraphElement>("engineStatusDetail");
  const summary = requiredElement<HTMLParagraphElement>("engineSummary");
  const summaryRow = requiredElement<HTMLDivElement>("engineSummaryRow");
  status.className = `engine-status ${state}`;
  summaryRow.className = `readiness-item ${state}`;

  if (state === "testing") {
    title.textContent = `${providerName}へ接続しています`;
    detail.textContent = "エンジンの応答を確認しています…";
    summary.textContent = "接続を確認しています…";
  } else if (state === "ready") {
    title.textContent = `${providerName}へ接続済み`;
    detail.textContent = readyDetail;
    summary.textContent = `${providerName}を使用できます。`;
  } else if (state === "unavailable") {
    title.textContent = `${providerName}が見つかりません`;
    detail.textContent =
      `${providerName}を起動し、準備完了後にもう一度お試しください。`;
    summary.textContent = `${providerName}を起動してください。`;
  } else {
    title.textContent = `${providerName}の接続を確認してください`;
    detail.textContent = "音声合成ソフトを起動して「接続を確認」を押します。";
    summary.textContent = "接続はまだ確認していません。";
  }
};
