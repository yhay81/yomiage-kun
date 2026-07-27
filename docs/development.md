# 開発ガイド

## Toolchain

- Rust 1.97 stable（リポジトリの `rust-toolchain.toml` で固定）
- Node.js 24 LTS
- npm（`package-lock.json` を使用）
- Tauri 2 の[前提パッケージ](https://v2.tauri.app/start/prerequisites/)

Windows は MSVC toolchain と WebView2、macOS は Xcode Command Line Tools が必要です。

## セットアップ

```powershell
git clone https://github.com/yhay81/yomiage-kun.git
Set-Location yomiage-kun
npm --prefix apps/desktop ci
cargo check --workspace --all-targets
npm --prefix apps/desktop run build
```

Windows では CMake が生成する libopus のパスが長くなるため、ワークスペースを短いパスへ置くか `CARGO_TARGET_DIR` を短くしてください。

```powershell
$env:CARGO_TARGET_DIR = 'C:\cargo-target\yomiage-kun'
cargo test --workspace
```

## 開発起動

```powershell
npm --prefix apps/desktop run tauri dev
```

AivisSpeech または VOICEVOX を起動すると接続テストと実際の読み上げを確認できます。実 Bot を使う統合テストでは、個人用のテストサーバーとテスト Bot を使用してください。

## 公式サイト

公式サイトは`site`にあるAstroプロジェクトです。

```powershell
npm --prefix site ci
npm --prefix site run dev
```

Cloudflareへ配信する前は、次のコマンドで型と静的ビルドを確認します。

```powershell
npm --prefix site run verify
```

## 品質チェック

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm --prefix apps/desktop run build
npm --prefix site run verify
npm --prefix apps/desktop run tauri build
```

ユニットテストは Discord トークンや起動中の TTS を必要としません。実ネットワークを使う確認は手動テストとして分離します。

## コード構成

- `yomiage-core`: Discord や UI に依存しない設定・正規化・TTS
- `yomiage-discord`: Serenity / Songbird による Bot
- `yomiage-desktop`: Tauri command、OS Keychain、設定、Web UI

新しい音声エンジンは `TtsProvider` を実装し、コアの factory と UI の provider selector へ追加します。秘密情報を trait の request やログへ含めないでください。

## Pull Request

変更範囲に応じたテストを追加し、上記チェックを通してください。生成済み bundle、秘密情報、個人用設定、`target`、`node_modules` は commit しません。
