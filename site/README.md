# 読み上げくん公式サイト

[Astro](https://astro.build/)で静的なページを作り、Cloudflare Workers Static Assetsから配信します。通常の閲覧ではWorkerのプログラムを実行しないため、配信リクエストは無料です。

## 手元で確認する

Node.js 24とnpmを使用します。

```powershell
npm ci
npm run dev
```

`http://localhost:4321` を開くと確認できます。

## 品質を確認する

```powershell
npm run verify
```

型、Astroの書き方、静的ビルドをまとめて確認します。

## Cloudflareへ配信する

Cloudflare Workers BuildsでGitHubリポジトリを接続し、次の値を使用します。

- ルートディレクトリ: `site`
- ビルドコマンド: `npm run build`
- 配信コマンド: `npm run deploy:preview`（本番ブランチ以外）
- 本番配信コマンド: `npx wrangler deploy`
- 本番ブランチ: `main`

`wrangler.jsonc`には静的ファイルのみを配信する設定があります。SSRやデータベースは使用しません。

初回は確認用の `workers.dev` URLで表示を確認してから、CloudflareのCustom Domainsで `yomiage.yusuke-hayashi.com` を接続します。
