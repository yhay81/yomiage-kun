import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, extname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { parse } from "parse5";

const projectRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const outputRoot = resolve(projectRoot, "dist");

function walk(directory) {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const path = join(directory, entry.name);
    return entry.isDirectory() ? walk(path) : [path];
  });
}

function visit(node, callback) {
  callback(node);
  for (const child of node.childNodes ?? []) {
    visit(child, callback);
  }
  if (node.content) {
    visit(node.content, callback);
  }
}

function attribute(node, name) {
  return node.attrs?.find((item) => item.name === name)?.value;
}

function localAssetExists(value) {
  const cleanPath = decodeURIComponent(value.split(/[?#]/, 1)[0]).replace(/^\/+/, "");
  return existsSync(resolve(outputRoot, cleanPath));
}

const htmlFiles = walk(outputRoot).filter((path) => extname(path) === ".html");
const failures = [];
let imageCount = 0;
let anchorCount = 0;

for (const htmlPath of htmlFiles) {
  const page = relative(outputRoot, htmlPath).replaceAll("\\", "/");
  const document = parse(readFileSync(htmlPath, "utf8"));
  const ids = new Set();
  const anchors = [];

  visit(document, (node) => {
    const id = attribute(node, "id");
    if (id) ids.add(id);

    if (node.tagName === "html" && attribute(node, "lang") !== "ja") {
      failures.push(`${page}: html要素のlangがjaではありません`);
    }

    if (node.tagName === "img") {
      imageCount += 1;
      const src = attribute(node, "src") ?? "";
      const alt = attribute(node, "alt") ?? "";
      const width = attribute(node, "width") ?? "";
      const height = attribute(node, "height") ?? "";

      if (!alt.trim()) failures.push(`${page}: ${src || "画像"} に説明がありません`);
      if (!/^\d+$/.test(width) || !/^\d+$/.test(height)) {
        failures.push(`${page}: ${src || "画像"} に固定サイズがありません`);
      }
      if (src && !/^https?:\/\//.test(src) && !localAssetExists(src)) {
        failures.push(`${page}: 画像 ${src} が出力先にありません`);
      }
    }

    if (node.tagName === "a") {
      const href = attribute(node, "href") ?? "";
      if (href.startsWith("#") && href.length > 1) anchors.push(href);
    }
  });

  for (const href of anchors) {
    anchorCount += 1;
    if (!ids.has(decodeURIComponent(href.slice(1)))) {
      failures.push(`${page}: ページ内リンク ${href} の移動先がありません`);
    }
  }
}

if (htmlFiles.length === 0) {
  failures.push("HTMLが1ページも生成されていません");
}
if (imageCount === 0) {
  failures.push("画像が1枚も生成されていません");
}

if (failures.length > 0) {
  console.error(failures.map((failure) => `- ${failure}`).join("\n"));
  process.exitCode = 1;
} else {
  console.log(
    `${htmlFiles.length}ページ、${imageCount}画像、${anchorCount}個のページ内リンクを確認しました。`,
  );
}
