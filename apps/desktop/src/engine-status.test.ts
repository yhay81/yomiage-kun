// @vitest-environment jsdom

import { beforeEach, describe, expect, it } from "vitest";
import { renderEngineStatus } from "./engine-status";

beforeEach(() => {
  document.body.innerHTML = `
    <div id="engineStatus"></div>
    <strong id="engineStatusTitle"></strong>
    <p id="engineStatusDetail"></p>
    <div id="engineSummaryRow"></div>
    <p id="engineSummary"></p>
  `;
});

describe("renderEngineStatus", () => {
  it("接続済みの音声数を表示する", () => {
    renderEngineStatus("ready", "VOICEVOX", "30種類の声を選べます。");

    expect(document.querySelector("#engineStatusTitle")?.textContent).toBe(
      "VOICEVOXへ接続済み",
    );
    expect(document.querySelector("#engineStatusDetail")?.textContent).toBe(
      "30種類の声を選べます。",
    );
    expect(document.querySelector("#engineSummaryRow")?.className).toContain(
      "ready",
    );
  });

  it("見つからないときに起動方法を案内する", () => {
    renderEngineStatus("unavailable", "AivisSpeech");

    expect(document.querySelector("#engineStatusTitle")?.textContent).toBe(
      "AivisSpeechが見つかりません",
    );
    expect(document.querySelector("#engineStatusDetail")?.textContent).toContain(
      "AivisSpeechを起動",
    );
  });
});
