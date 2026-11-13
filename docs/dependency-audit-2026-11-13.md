# 2026-11-13 依存関係監査

## 状態

この変更は依存関係修正の途中段階です。Rust の監査は下記の2件が未解決のため失敗します。新しい `deny.toml` の例外は追加していません。暗号依存を解決するまでリリース可能とは扱いません。

基準は main `799d111b0c1cafb7056558191d29acd96134505a` と [Security run 43433775145](https://github.com/yhay81/yomiage-kun/actions/runs/43433775145) です。[PR #27](https://github.com/yhay81/yomiage-kun/pull/27) の rustls 更新も含みますが、残りの依存関係と npm 監査の修正が必要でした。

## 修正した依存関係

| 対象 | 変更 | 解消する指摘 |
| --- | --- | --- |
| rustls | 0.23.45 → 0.23.46 | RUSTSEC-2026-0346 / 0347 / 0348 / 0349 |
| rustls-webpki | 0.103.15 → 0.103.17 | RUSTSEC-2026-0382 |
| tokio | 1.53.1 → 1.54.1、manifest の最低版を 1.53.3 に設定 | RUSTSEC-2026-0386 |
| tauri-plugin-updater | Rust / JavaScript ともに 2.10.1 → 2.14.1 | zip 4.6.1 → 9.5.1 により RUSTSEC-2026-0367 / 0368 / 0369 / 0429 |
| Tauri の間接依存 | updater が要求する Tauri 2.12 系へ更新 | 不要になった unic 系の5件の例外を削除 |
| Frontend | Vite 8.3.8、Vitest 4.1.12、undici / source-map-js など | npm 監査で検出された high / moderate の指摘 |
| 公式サイト | Astro 7.4.11、sharp 0.35.5、Wrangler 4.162.0 と間接依存 | npm 監査で検出された high / moderate の指摘 |

Rust の直接依存は既存のメジャー系列を維持し、ロックファイルを含めて更新しました。Songbird の既存 ringbuf パッチは保持しています。DAVE / OpenMLS も互換範囲の davey 0.1.5 / openmls 0.8.3 へ更新しています。

## 自動更新の利用経路

UI は `apps/desktop/src/main.ts` の `check()` と `Update.downloadAndInstall()` を使用し、Rust 側は `tauri_plugin_updater::Builder` を登録します。既存の公開鍵と HTTPS の GitHub Releases エンドポイントを保持しています。

updater 2.14.1 の公開ソースでは、ダウンロード後に署名を確認してからインストールします。Windows の ZIP は `extract_zip()` → `ZipArchive::extract()` を通るため、ZIP の指摘を単に無関係とは扱えません。修正済み zip 9.5.1 へ更新し、既存の ZIP 対応機能を保持しました。現在の Windows 配布は NSIS の直接 `.exe`、macOS は `.app.tar.gz` です。どちらの形式も updater 2.14.1 が扱います。macOS の展開には tar / flate2 が使われます。

参照: [updater 2.14.1 のソース](https://docs.rs/crate/tauri-plugin-updater/2.14.1/source/src/updater.rs)、[依存 manifest](https://docs.rs/crate/tauri-plugin-updater/2.14.1/source/Cargo.toml)、[ZIP advisory](https://rustsec.org/advisories/RUSTSEC-2026-0368.html)。ビルドの成功だけで実際の署名付き更新・再起動の動作確認が済んだとは扱いません。

## 残るブロッカー

### crypto-bigint 0.5.5 — RUSTSEC-2026-0387

依存経路は `yomiage-desktop → yomiage-discord → songbird → davey / OpenMLS → p256 → elliptic-curve 0.13.8 → crypto-bigint 0.5.5` です。HPKE の p384 / k256 からの経路もあります。

DAVE v1 の選択する cipher suite は `MLS_128_DHKEMP256_AES128GCM_SHA256_P256` です。Songbird は接続時に `DaveSession::new()` と `create_key_package()` を呼び、davey は `SignatureKeyPair::new()` と `OpenMlsRustCrypto` を使用します。したがって暗号処理は実際の Discord 音声接続で使用されます。攻撃成立の確認はしていませんが、秘密値の処理時間やマイクロアーキテクチャ上の観測による情報漏えいのリスクを否定できません。

[RustSec](https://rustsec.org/advisories/RUSTSEC-2026-0387.html) の修正版は crypto-bigint >=0.7.6 です。elliptic-curve 0.13.8 は 0.5 系を要求し、lockfile だけで 0.7 系へ置換できません。調査時点の davey 0.2.0 と OpenMLS の最新公開 crypto / credential backend 0.6.0 も p256 0.13 を要求しています。p256 0.14 と hpke-rs-rust-crypto 0.8.0 は公開済みですが、DAVE と OpenMLS の複数 backend を含む移行が必要です。

解消には上流が対応したリリース、または別途レビューした互換移行が必要です。後者では DAVE の鍵生成・署名・MLS commit/welcome・暗号化の相互運用テストと実 Discord 音声接続の確認が必要です。独自の暗号演算修正やバージョン番号の偽装は行っていません。

参照: [davey 0.1.5 session](https://docs.rs/crate/davey/0.1.5/source/src/session.rs)、[davey 0.2.0 manifest](https://docs.rs/crate/davey/0.2.0/source/Cargo.toml)、[OpenMLS crypto 0.6.0 manifest](https://docs.rs/crate/openmls_rust_crypto/0.6.0/source/Cargo.toml)、[elliptic-curve 0.13.8 manifest](https://docs.rs/crate/elliptic-curve/0.13.8/source/Cargo.toml)。

### pin-utils 0.1.0 — RUSTSEC-2026-0441

依存経路は `davey → openmls (js feature) → fluvio-wasm-timer 0.2.5 → pin-utils` です。これは既知の攻撃を報告する advisory ではなく、保守終了の指摘です。fluvio-wasm-timer の公開版は 0.2.5 のみで、安全版への単純な更新はありません。

OpenMLS の `key_packages/lifetime.rs` は wasm32 だけで fluvio の時刻 API を使い、配布対象の Windows/macOS では `std::time` を使用します。ただし davey が js feature を有効にするため依存グラフには含まれます。次の対応は上流で js 依存を wasm32 に限定するか、fluvio の pin マクロを保守された実装へ移行することです。対象プラットフォームとタイマーの挙動を検証してから更新します。

参照: [advisory](https://rustsec.org/advisories/RUSTSEC-2026-0441.html)、[OpenMLS 0.8.3 lifetime](https://docs.rs/crate/openmls/0.8.3/source/src/key_packages/lifetime.rs)、[fluvio-wasm-timer 0.2.5](https://docs.rs/crate/fluvio-wasm-timer/0.2.5/source/)。

## 再検証

CI と同じ Rust 1.97.0、Node.js 24 で実行します。`cargo deny` は上記2件が解消するまで失敗することが期待されます。既存の他の5件の RustSec 例外はこの修正でも残っています。

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo deny --all-features --locked check advisories licenses sources
node .github/scripts/verify-release-version.mjs
npm --prefix apps/desktop ci --no-audit
npm --prefix apps/desktop audit --audit-level=high
npm --prefix apps/desktop run lint
npm --prefix apps/desktop test
npm --prefix apps/desktop run build
npm --prefix site ci --no-audit
npm --prefix site audit --audit-level=high
npm --prefix site run verify
```

実 Discord / TTS 接続、署名付きインストーラ生成、実アプリの自動更新は別途手動検証が必要です。
