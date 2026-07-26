# リリース手順

## 前提

GitHub Actions が Windows x64、macOS Apple Silicon、macOS Intel の bundle を作成します。公開用リリースではコード署名を必須とします。

必要な GitHub Actions secrets:

- Tauri updater を使う場合の署名鍵とパスワード
- macOS Developer ID Application 証明書、公証用 Apple ID / App Store Connect API key
- Windows Authenticode 証明書またはクラウド署名サービスの資格情報

秘密値はリポジトリや workflow の平文へ追加しません。

## バージョン更新

次の値を同じ SemVer に揃えます。

- workspace `Cargo.toml` の `workspace.package.version`
- `apps/desktop/package.json`
- `apps/desktop/src-tauri/tauri.conf.json`
- `CHANGELOG.md`

## 検証

1. CI がすべて成功していることを確認します。
2. Windows と macOS の成果物をクリーン環境へインストールします。
3. トークン保存、音声エンジン接続、Bot 起動、`/join`、読み上げ、`/leave`、自動起動を確認します。
4. 署名と macOS notarization を確認します。

## 公開

`vX.Y.Z` の annotated tag を作成して push すると release workflow が bundle を作ります。最初は Draft Release として生成し、成果物とチェックサムを確認してから公開します。

署名 secrets が設定されていない fork では CI の compile check のみ利用してください。未署名 bundle を公式 Release として公開しないでください。
