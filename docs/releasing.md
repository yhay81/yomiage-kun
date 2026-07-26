# リリース手順

## 前提

GitHub Actions が Windows x64、macOS Apple Silicon、macOS Intel の bundle を作成します。公開用リリースではコード署名を必須とします。

必要な GitHub Actions secrets:

- `TAURI_SIGNING_PRIVATE_KEY` / `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`
- `WINDOWS_CERTIFICATE` / `WINDOWS_CERTIFICATE_PASSWORD`
- `APPLE_CERTIFICATE` / `APPLE_CERTIFICATE_PASSWORD` / `APPLE_SIGNING_IDENTITY`
- `APPLE_ID` / `APPLE_PASSWORD` / `APPLE_TEAM_ID`

秘密値はリポジトリや workflow の平文へ追加しません。

Tauri updaterの秘密鍵は、GitHub Actions secretsとは別に暗号化して安全な場所へバックアップします。この鍵を失うと、既存利用者へ自動更新を配れなくなります。公開鍵だけを`tauri.conf.json`へ記録します。

Windows証明書はPFXをBase64化して登録します。macOSはDeveloper ID Application証明書とApple IDのアプリ用パスワードを使います。無料のApple Developerアカウントでは公証できないため、公式版の配布には有料アカウントが必要です。

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
5. `SHA256SUMS.txt`と`latest.json`がドラフトへ追加されていることを確認します。
6. アプリ内の更新確認が公開済みの一つ前の版から動作することを確認します。

## 公開

`vX.Y.Z` のannotated tagを作成してpushするとrelease workflowがbundleを作ります。署名用secretが一つでも不足している場合は、未署名版を作らずに停止します。成果物はDraft Releaseとして生成し、署名、公証、チェックサム、実機確認が終わってから公開します。

署名 secrets が設定されていない fork では CI の compile check のみ利用してください。未署名 bundle を公式 Release として公開しないでください。
