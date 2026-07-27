// @vitest-environment jsdom

import { describe, expect, it } from "vitest";
import { appMarkup } from "./ui";

describe("初回設定画面", () => {
  it("専門的な話者番号ではなく声の名前を選べる", () => {
    document.body.innerHTML = appMarkup;

    const voice = document.querySelector<HTMLSelectElement>("#speakerId");
    expect(voice).not.toBeNull();
    expect(document.body.textContent).not.toContain("話者番号");
    expect(document.body.textContent).toContain("自動で見つける");
    expect(document.body.textContent).toContain("この声を試す");
    expect(document.querySelector("#pitch")).not.toBeNull();
  });

  it("診断情報と使い方へ迷わず進める", () => {
    document.body.innerHTML = appMarkup;

    expect(document.querySelector("#exportDiagnostics")).not.toBeNull();
    expect(document.querySelector("#openDocs")).not.toBeNull();
    expect(document.querySelector("#clearToken")).not.toBeNull();
  });
});
