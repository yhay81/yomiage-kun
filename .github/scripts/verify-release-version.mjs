import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const root = resolve(import.meta.dirname, "../..");
const read = (path) => readFileSync(resolve(root, path), "utf8");
const fail = (message) => {
  console.error(`リリース情報の検証に失敗しました: ${message}`);
  process.exit(1);
};

const desktopPackage = JSON.parse(read("apps/desktop/package.json"));
const tauriConfig = JSON.parse(read("apps/desktop/src-tauri/tauri.conf.json"));
const cargoVersion = read("Cargo.toml").match(
  /^\[workspace\.package\][\s\S]*?^version\s*=\s*"([^"]+)"/m,
)?.[1];
const version = desktopPackage.version;

if (!/^\d+\.\d+\.\d+$/.test(version)) fail(`不正なアプリ版です: ${version}`);
if (cargoVersion !== version) {
  fail(`Cargo.toml (${cargoVersion}) と package.json (${version}) が一致しません`);
}
if (tauriConfig.version !== version) {
  fail(`tauri.conf.json (${tauriConfig.version}) と package.json (${version}) が一致しません`);
}

const requestedTag = process.argv[2];
if (requestedTag) {
  const expectedTag = `v${version}`;
  if (requestedTag !== expectedTag) {
    fail(`実行対象 ${requestedTag} はアプリ版 ${expectedTag} と一致しません`);
  }
  const changelog = read("CHANGELOG.md");
  if (!changelog.includes(`## [${version}] - `)) {
    fail(`CHANGELOG.md に公開版 ${version} の日付付き見出しがありません`);
  }
}

console.log(`リリース情報は一致しています: v${version}`);
