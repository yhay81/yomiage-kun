# 読み上げくん

Discordに届いたメッセージを、やさしい日本語の声で読み上げるアプリです。

WindowsとMacで使えます。

[公式サイト](https://yhay81.github.io/yomiage-kun/) ・ [ダウンロード](https://github.com/yhay81/yomiage-kun/releases) ・ [くわしい始め方](docs/getting-started.md)

[![読み上げくん](https://yhay81.github.io/yomiage-kun/og.jpg)](https://yhay81.github.io/yomiage-kun/)

## こんなときに便利です

- ゲーム中に、手を止めずにメッセージを聞きたい
- 作業中に、仲間からのメッセージを聞き逃したくない
- 自分の好きな声や話す速さで読み上げてほしい
- みんなで使うための月額サービスを契約したくない

読み上げくんは、あなたのパソコンで動きます。

開発者が用意した有料の中継サービスは使いません。

## 用意するもの

- Windows 11、またはmacOS 13以降のパソコン
- Discord
- AivisSpeech、またはVOICEVOX
- 読み上げくんを追加できるDiscordサーバー

AivisSpeechとVOICEVOXは、文章を声に変えてくれる無料のアプリです。どちらか一つを使います。

## はじめ方

1. [ダウンロードページ](https://github.com/yhay81/yomiage-kun/releases)から、Windows版またはMac版を入れます。
2. AivisSpeechまたはVOICEVOXを入れて、起動します。
3. 読み上げくんを開き、画面の案内に沿ってDiscordに「読み上げ役」を作ります。
4. Discordから発行された合言葉を、読み上げくんに貼り付けます。
5. 「読み上げを始める」を押し、Discordで `/join` と入力します。

Discordの準備は最初の一度だけです。画像付きの手順は[Discordの準備](docs/discord-setup.md)で確認できます。

## ふだんの使い方

1. AivisSpeechまたはVOICEVOXを起動します。
2. 読み上げくんで「読み上げを始める」を押します。
3. Discordの通話に入り、メッセージ欄へ `/join` と入力します。

読み上げを終えるときは `/leave`、今の読み上げを飛ばすときは `/skip` と入力します。

## 安心して使うために

- メッセージから声を作る処理は、あなたのパソコンの中で行います。
- 読み上げた声を、パソコンへ音声ファイルとして残しません。
- Discordから発行された合言葉は、WindowsまたはMacの安全な保管場所に保存します。
- 開発者がメッセージや合言葉を受け取ることはありません。

## 困ったとき

- [うまく動かないときの確認](docs/troubleshooting.md)
- [Discordの準備](docs/discord-setup.md)
- [不具合や要望を伝える](https://github.com/yhay81/yomiage-kun/issues)

## 開発に参加したい方へ

アプリの仕組みや開発方法は、[開発者向けの案内](docs/development.md)と[協力方法](CONTRIBUTING.md)をご覧ください。

## ライセンス

[MIT License](LICENSE)
