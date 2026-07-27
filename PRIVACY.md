# プライバシーについて

読み上げくんは、利用者のパソコンで動くアプリです。広告、利用状況の分析、開発者が運営する中継サーバーはありません。

## 通信する相手

読み上げくんが通信するのは、利用者が使うために指定した次のサービスだけです。

- Discord — メッセージを受け取り、通話へ音声を届けます
- AivisSpeechまたはVOICEVOX — メッセージを音声に変えます。通常は同じパソコンの中で通信します
- GitHub — 新しい版があるかを確認します

開発者がメッセージ、音声、Discordの合言葉を受け取ることはありません。広告会社や分析サービスへ情報を送りません。

利用者が自分で外部のAivisSpeechまたはVOICEVOX接続先を設定した場合は、読み上げる文章がその接続先へ送られます。接続先の運営者と利用条件を確認してから使用してください。

## 保存する情報

Discordの合言葉は、Windowsの資格情報マネージャーまたはMacのキーチェーンへ保存します。設定や動作記録へ合言葉と読み上げた文章は残しません。

利用者が **診断情報を保存** を押した場合だけ、アプリの版、OS、音声設定、接続状態を利用者が選んだファイルへ保存します。合言葉と読み上げた文章は含まれません。

## 公式サイト

公式サイトはCloudflareから静的なページとして配信します。広告、Cookieを使うアクセス解析、利用者を追跡する仕組みは使用しません。ページを届けるために必要な一般的な通信記録は、Cloudflare側で短期間処理される場合があります。

## 利用するサービスの案内

- [Discordのプライバシーポリシー](https://discord.com/privacy)
- [Aivis Projectの利用規約](https://hub.aivis-project.com/terms-of-service/)・[プライバシーポリシー](https://hub.aivis-project.com/privacy-policy)
- [VOICEVOXの利用規約](https://voicevox.hiroshiba.jp/term/)
- [GitHubのプライバシーについて](https://docs.github.com/ja/site-policy/privacy-policies/github-general-privacy-statement)
- [Cloudflareのプライバシーポリシー](https://www.cloudflare.com/privacypolicy/)

より詳しい仕組みは、[セキュリティとプライバシー](docs/security.md)をご覧ください。

## お問い合わせ

秘密情報やDiscordの合言葉を含めずに、[GitHub Issues](https://github.com/yhay81/yomiage-kun/issues)からご連絡ください。脆弱性については[セキュリティポリシー](SECURITY.md)をご覧ください。
