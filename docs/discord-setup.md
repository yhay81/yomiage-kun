# Discord Bot セットアップ

Yomiage-kun は利用者ごとに専用 Bot を作る方式です。開発者の共有 Bot を使わないため、ホスティング費用や利用制限がなく、トークンも自分の PC だけに保存されます。

## 1. Application を作る

1. Yomiage-kun の **Discord Developer Portal を開く**を押します。
2. **New Application** を選び、分かりやすい名前を付けます。例: `My Yomiage-kun`
3. 利用規約に同意し、**Create** を押します。

## 2. Bot を作り、Intent を有効にする

1. 左側の **Bot** を開きます。
2. Bot がまだなければ **Add Bot** を選びます。
3. **Privileged Gateway Intents** にある **Message Content Intent** を有効にします。
4. **Save Changes** を押します。

Yomiage-kun は投稿本文を受け取るため Message Content Intent を必要とします。Presence Intent と Server Members Intent は不要です。

## 3. トークンを保存する

1. Bot ページの **Reset Token** を選び、表示されたトークンをコピーします。
2. Yomiage-kun の **Bot トークン**欄へ貼り付けます。
3. **検証して保存**を押します。

検証に成功するとトークンは Windows Credential Manager または macOS Keychain に保存され、通常の設定ファイルには書かれません。

> [!WARNING]
> Bot トークンはパスワードと同じです。スクリーンショット、Git、チャット、問い合わせ本文へ貼らないでください。漏えいした可能性があれば Developer Portal で直ちに Reset Token を実行してください。

## 4. サーバーへ追加する

1. 検証後に有効になる **サーバーへ追加**を押します。
2. 追加先のサーバーを選びます。
3. 表示される権限を確認して承認します。

生成される URL は `bot` と `applications.commands` スコープを使用し、次の権限だけを要求します。

- View Channels
- Send Messages
- Connect
- Speak

Administrator 権限は不要です。Discord サーバーのチャンネル権限で、読み上げを許可する範囲をさらに限定できます。

## 5. 起動を確認する

1. 音声エンジンの **接続テスト**を成功させます。
2. **Bot を開始**を押します。
3. Discord のメンバー一覧で Bot がオンラインになったことを確認します。
4. 自分がボイスチャンネルへ入り、対象テキストチャンネルで `/join` を実行します。

Slash Command が見えない場合は、招待時に `applications.commands` が含まれているか確認し、数分待って Discord クライアントを再起動してください。

## 複数サーバーで使う

同じ Bot を自分が管理する複数サーバーへ追加できます。Yomiage-kun はサーバーごとに独立した読み上げキューを作ります。1 台の PC と 1 つの Bot トークンで同時に複数サーバーへ接続できますが、音声合成の速度と回線性能が上限になります。

家族やチームで利用する場合も、トークン共有は避け、ホスト担当者が自分の PC でアプリを動かしてください。常時稼働が必要な場合は専用 PC が必要です。公式の共有ホスティングは提供していません。
