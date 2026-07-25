# 旧版からの移行

2018 年版は Python、discord.py、BouyomiChan、ファイル監視を組み合わせた実験的 Bot でした。v2 は互換性を持たない全面刷新です。

| 旧版 | v2 |
|---|---|
| Python スクリプトを直接編集・実行 | Windows / macOS デスクトップアプリ |
| BouyomiChan の一時ファイル連携 | AivisSpeech / VOICEVOX HTTP API |
| Prefix command | Discord Slash Command |
| 平文設定へ token を記述 | OS 資格情報ストア |
| グローバルな再生状態 | Guild ごとの有界キュー |
| 実行環境を手作業で構築 | Release installer |

## 移行手順

1. 旧 Bot を停止します。
2. 同じ Discord Application を使う場合は Developer Portal で Message Content Intent を有効にします。
3. v2 をインストールし、既存 Bot トークンをアプリで検証・保存します。
4. アプリの招待ボタンから Bot を再招待し、Slash Command の scope と最小権限を適用します。
5. AivisSpeech または VOICEVOX を設定し、接続テストを実行します。

旧版の設定ファイルは自動インポートしません。平文 token が残っている場合は削除し、安全のため Developer Portal で Reset Token することを推奨します。

旧 `main.py` は Git 履歴から参照できます。v2 の既定ブランチには含めません。
