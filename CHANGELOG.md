# Changelog

このプロジェクトの重要な変更を記録します。形式は [Keep a Changelog](https://keepachangelog.com/ja/1.1.0/) に基づき、バージョンは [Semantic Versioning](https://semver.org/lang/ja/) に従います。

## [Unreleased]

## [0.1.3] - 2026-11-13

0.1.2 は配布ビルドで Tauri の版不一致を検出したため未配布です。以下の変更は 0.1.3 で初めて配布します。

### Security

- Rustls、WebPKI、Tokio、ZIP の脆弱性を修正済みの依存版へ更新
- DAVE / OpenMLS の暗号バックエンドに上流の互換移行を取り込み、crypto-bigint の問題を解消
- Windows / macOS で不要な WASM タイマー依存を外し、pin-utils の保守終了の指摘を解消
- デスクトップのフロントエンドと公式サイトの依存関係を更新し、npm 監査の指摘を解消

### Fixed

- Tauri の Rust 本体・JavaScript API・CLI を互換性のある版へ揃え、配布ビルドの停止を解消
- 通常 CI のリリース情報検証でも Tauri 本体と JavaScript API の major/minor 不一致を検出

### Changed

- Tauri と自動更新プラグインを更新し、既存の更新署名と設定形式を維持
- 標準の暗号テストベクトルと、旧版 DAVE との参加・鍵更新・音声暗号化の相互運用試験を CI に追加

## [0.1.1] - 2026-08-09

### Added

- AivisSpeech / VOICEVOX の自動検出
- 音声合成ソフトから取得した声の名前一覧と試聴
- トークンや投稿本文を含まない診断情報の書き出し
- 署名で保護されたアプリ内自動更新
- フロントエンド、公式サイト、依存関係とライセンスの自動検査
- Windows / macOSと音声合成ソフトの対応環境表
- 声の高さ調整とDiscord連携解除

### Changed

- 製品名を「読み上げくん」に統一
- 日常操作をスクロール不要の1画面に集約し、初回設定を別画面へ整理
- Windowsインストーラを日本語化
- 旧英語名版を日本語名版へ自動で置き換え、設定とトークンを引き継ぐ移行処理
- リリース成果物をWindows用1個、Mac用2個のインストーラへ簡素化
- Rust、Tauri、TypeScript、Viteと直接依存パッケージを安定最新版へ更新
- Discord通信をWindows / macOSの証明書ストアを使うTLS構成へ更新
- Discordの管理操作をサーバー管理者へ限定し、終了時に音声キューを確実に破棄
- 設定を原子的に保存し、破損時はバックアップまたは初期値へ自動復旧
- リリースタグ、アプリ版、変更履歴の一致を公開前に自動検査

### Security

- 署名secretsが設定されている場合にWindows署名とmacOS公証を必須検証するリリースゲートを追加(コード署名自体は導入準備中で、現行版は`SHA256SUMS.txt`で検証)
- CodeQL、依存関係レビュー、脆弱性・ライセンス検査を追加
- 音声エンジンから受信する音声データに16 MiBの上限を追加

## [0.1.0] - 2026-07-26

### Added

- Tauri 2 による Windows / macOS デスクトップアプリ
- 利用者自身の Discord Bot を検証・保存・招待するセットアップ UI
- AivisSpeech / VOICEVOX のインメモリ音声合成
- Discord Slash Commands: `/join`、`/leave`、`/skip`、`/status`
- OS 資格情報ストア、自動起動、二重起動防止
- Guild ごとの有界キュー、テキスト正規化、合成キャッシュ
- CI、クロスプラットフォーム release workflow、利用者・開発者ドキュメント

### Fixed

- 音声エンジン停止時に内部HTTPエラーを表示せず、起動と再試行を案内
- WindowsとmacOSでアプリ本体と一体化したカスタムタイトルバーを使用
- Apple Silicon runnerからmacOS Intel版をビルドする際のRust target不整合

### Removed

- Python、BouyomiChan、一時ファイル監視に依存する旧実装

[Unreleased]: https://github.com/yhay81/yomiage-kun/compare/v0.1.3...HEAD
[0.1.3]: https://github.com/yhay81/yomiage-kun/compare/v0.1.1...v0.1.3
[0.1.1]: https://github.com/yhay81/yomiage-kun/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/yhay81/yomiage-kun/releases/tag/v0.1.0
