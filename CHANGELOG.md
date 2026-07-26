# Changelog

このプロジェクトの重要な変更を記録します。形式は [Keep a Changelog](https://keepachangelog.com/ja/1.1.0/) に基づき、バージョンは [Semantic Versioning](https://semver.org/lang/ja/) に従います。

## [Unreleased]

### Changed

- 製品名を「読み上げくん」に統一
- 日常操作をスクロール不要の1画面に集約し、初回設定を別画面へ整理
- WindowsのNSIS / MSIインストーラを日本語化
- 旧英語名版を日本語名版へ自動で置き換え、設定とトークンを引き継ぐ移行処理
- リリース成果物をWindows用1個、Mac用2個のインストーラへ簡素化

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

[Unreleased]: https://github.com/yhay81/yomiage-kun/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yhay81/yomiage-kun/releases/tag/v0.1.0
