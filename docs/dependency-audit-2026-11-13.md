# 2026-11-13 依存関係監査

## 状態

配布対象の Windows x64 / macOS Apple Silicon / macOS Intel について、今回失敗した Rust の12件を解消しました。新しい監査例外は追加していません。既存例外は不要な9件を削除し、Songbird の `derivative` 保守終了（RUSTSEC-2024-0388）の1件だけを維持しています。npm 監査も両プロジェクトで指摘0件です。

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

## 暗号依存の解消

元の経路は `Songbird → davey / OpenMLS → p256 / p384 / k256 → elliptic-curve 0.13 → crypto-bigint 0.5.5` でした。DAVE の P-256 署名・鍵交換で実際に使うため、監査除外にはできません。[RUSTSEC-2026-0387](https://rustsec.org/advisories/RUSTSEC-2026-0387.html) は 0.7.6 以降で修正されています。

単純な lockfile 更新は親のバージョン制約で不可能です。公開済み davey 0.2.0 と OpenMLS crypto / credential 0.6.0 も古い p256 を要求するため、メジャー更新だけでは解消しません。DAVE の削除は Discord 音声接続の暗号化を壊します。暗号演算を独自に書き換える案も採用していません。

OpenMLS 上流で取り込まれた [暗号クレート移行 #2249](https://github.com/openmls/openmls/commit/055a2b4b06b595585783ee488e28eebd62212e2a) と公開済み HPKE 0.8 を、既存の OpenMLS 0.8 API を維持して取り込みました。`davey`、`openmls_rust_crypto`、`openmls_basic_credential` の3クレートをソースとライセンス付きで固定し、変更は依存宣言・乱数/署名API・長さ検証に限定しています。暗号演算は公開済み RustCrypto / HPKE 実装が担当し、すべての経路が修正済み crypto-bigint 0.7.11 を使います。

ソースのコミット・アーカイブの SHA-256・差分の範囲・パッチを外す条件は [vendor/CRYPTO-PATCHES.md](../vendor/CRYPTO-PATCHES.md) に記録しています。この取り込みは独立した暗号監査の代替ではありません。

RFC 6979 / 8032 / 9180 の外部テストベクトルで署名・検証・HPKE鍵導出・復号・エクスポートを検証し、不正な長さと改ざんを拒否することを確認します。さらに無変更の公開版 davey 0.1.5 と修正版を別プロセスで接続し、双方が commit/welcome を送る組み合わせ、3人目の参加による鍵更新、音声の双方向暗号化、改ざん・再送の拒否を検証します。旧版はテスト専用の一時プロジェクトに隔離し、配布物には含めません。この相互運用試験を Windows / macOS の CI に追加しています。

## pin-utils の解消

`davey → openmls (js) → fluvio-wasm-timer → pin-utils` は WASM の時刻取得用でした。OpenMLS の [lifetime.rs](https://docs.rs/crate/openmls/0.8.3/source/src/key_packages/lifetime.rs) は Windows / macOS で `std::time` を使います。davey の `js` feature を `cfg(target_arch = "wasm32")` に限定し、3つの配布対象からタイマーごと取り除きました。[RUSTSEC-2026-0441](https://rustsec.org/advisories/RUSTSEC-2026-0441.html) を除外していません。

非配布対象の WASM 用には Cargo.lock に pin-utils が残ります。WASM ビルドを安全とする主張はしておらず、WASM を配布する場合はこの依存を別途解消する必要があります。

## 再検証

CI と同じ Rust 1.97.0、Node.js 24 で実行します。監査は配布対象3つを `deny.toml` の既存設定で検証します。

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
python .github/scripts/check-dave-interop.py
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

実 Discord / TTS 接続、クリーンな実機へのインストール、旧版アプリからの更新と再起動は、この自動試験では確認できません。リリースでは別途、3対象のビルド成果物・自動更新署名・チェックサムを確認します。OSコード署名とmacOS公証は既存方針どおり証明書未登録のため実施しません。
