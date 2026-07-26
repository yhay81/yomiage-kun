// @vitest-environment jsdom

import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";

const siteRoot = resolve(process.cwd(), "../../site");
const html = readFileSync(resolve(siteRoot, "index.html"), "utf8");

describe("公式サイト", () => {
  it("すべての画像に説明と固定サイズがある", () => {
    document.documentElement.innerHTML = html;
    const images = [...document.querySelectorAll<HTMLImageElement>("img")];
    expect(images.length).toBeGreaterThan(0);

    for (const image of images) {
      expect(image.alt.trim(), image.src).not.toBe("");
      expect(image.getAttribute("width"), image.src).toMatch(/^\d+$/);
      expect(image.getAttribute("height"), image.src).toMatch(/^\d+$/);
      const source = image.getAttribute("src");
      if (source && !source.startsWith("http")) {
        expect(existsSync(resolve(siteRoot, source)), source).toBe(true);
      }
    }
  });

  it("ページ内リンクの移動先が存在する", () => {
    document.documentElement.innerHTML = html;
    for (const link of document.querySelectorAll<HTMLAnchorElement>('a[href^="#"]')) {
      const id = link.hash.slice(1);
      expect(document.getElementById(id), link.hash).not.toBeNull();
    }
  });
});
