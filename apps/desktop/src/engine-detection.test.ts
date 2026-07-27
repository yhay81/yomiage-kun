import { describe, expect, it } from "vitest";
import { replaceDetectedEngines } from "./engine-detection";

type Provider = "aivis_speech" | "voicevox";
interface Engine {
  provider: Provider;
  version: string;
}

describe("音声エンジンの再検出", () => {
  it("停止済みエンジンをキャッシュから除く", () => {
    const cache = new Map<Provider, Engine>([
      ["aivis_speech", { provider: "aivis_speech", version: "1.0" }],
    ]);

    const selected = replaceDetectedEngines(cache, [], "aivis_speech");

    expect(selected).toBeUndefined();
    expect(cache.size).toBe(0);
  });

  it("選択中のエンジンを優先する", () => {
    const cache = new Map<Provider, Engine>();
    const engines: Engine[] = [
      { provider: "aivis_speech", version: "1.0" },
      { provider: "voicevox", version: "2.0" },
    ];

    expect(replaceDetectedEngines(cache, engines, "voicevox")).toEqual(
      engines[1],
    );
  });
});
