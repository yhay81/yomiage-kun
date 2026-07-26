export type EngineState = "idle" | "testing" | "ready" | "unavailable";

const requiredElement = <T extends HTMLElement>(id: string): T => {
  const value = document.getElementById(id);
  if (!value) throw new Error(`Missing engine status element: ${id}`);
  return value as T;
};

export const renderEngineStatus = (
  state: EngineState,
  providerName: string,
): void => {
  const status = requiredElement<HTMLDivElement>("engineStatus");
  const title = requiredElement<HTMLElement>("engineStatusTitle");
  const detail = requiredElement<HTMLParagraphElement>("engineStatusDetail");
  status.className = `engine-status ${state}`;

  if (state === "testing") {
    title.textContent = `${providerName}へ接続しています`;
    detail.textContent = "エンジンの応答を確認しています…";
  } else if (state === "ready") {
    title.textContent = `${providerName}へ接続済み`;
    detail.textContent = "読み上げを開始できます。";
  } else if (state === "unavailable") {
    title.textContent = `${providerName}が見つかりません`;
    detail.textContent =
      `${providerName}を起動し、準備完了後にもう一度お試しください。`;
  } else {
    title.textContent = `${providerName}の接続を確認してください`;
    detail.textContent = "エンジンを起動して「接続テスト」を押します。";
  }
};
