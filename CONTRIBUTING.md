# Contributing

Issue や Pull Request を歓迎します。やり取りは日本語・英語のどちらでも構いません。

## 始める前に

- バグ報告では OS、アプリ版、音声エンジン、再現手順、期待結果を記載してください。
- Bot token、個人の投稿本文、サーバー ID、個人情報を添付しないでください。
- 大きな機能や互換性を壊す変更は、実装前に Discussion または Issue で設計を相談してください。

## 開発フロー

1. fork からトピックブランチを作ります。
2. [開発ガイド](docs/development.md) に従ってセットアップします。
3. 変更に対応するテストとドキュメントを更新します。
4. format、clippy、test、frontend build を実行します。
5. 目的、変更内容、確認結果を Pull Request に記載します。

依存関係は lockfile と一緒に更新し、不要な package manager や runtime を追加しないでください。秘密情報、生成 bundle、`target`、`node_modules` を commit しないでください。

このプロジェクトへ提出した貢献は MIT License の下で提供されることに同意したものとします。
