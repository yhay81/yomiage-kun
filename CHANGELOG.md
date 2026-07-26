# Changelog

このプロジェクトの重要な変更を記録します。形式は [Keep a Changelog](https://keepachangelog.com/ja/1.1.0/) に基づき、バージョンは [Semantic Versioning](https://semver.org/lang/ja/) に従います。

## [Unreleased]

## [0.1.1] - 2026-07-26

### Added

- AivisSpeech / VOICEVOX の自動検出
- 音声合成ソフトから取得した声の名前一覧と試聴
- トークンや投稿本文を含まない診断情報の書き出し
- 署名で保護されたアプリ内自動更新
- フロントエンド、公式サイト、依存関係とライセンスの自動検査
- Windows / macOSと音声合成ソフトの対応環境表

### Changed

- 製品名を「読み上げくん」に統一
- 日常操作をスクロール不要の1画面に集約し、初回設定を別画面へ整理
- Windowsインストーラを日本語化
- 旧英語名版を日本語名版へ自動で置き換え、設定とトークンを引き継ぐ移行処理
- リリース成果物をWindows用1個、Mac用2個のインストーラへ簡素化
- Rust、Tauri、TypeScript、Viteと直接依存パッケージを安定最新版へ更新
- Discord通信をWindows / macOSの証明書ストアを使うTLS構成へ更新

### Security

- Windowsのコード署名とmacOSの署名・公証をリリース条件に追加
- CodeQL、依存関係レビュー、脆弱性・ライセンス検査を追加

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

[Unreleased]: https://github.com/yhay81/yomiage-kun/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/yhay81/yomiage-kun/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/yhay81/yomiage-kun/releases/tag/v0.1.0
