# Code signing policy

読み上げくんのWindows版は、公開前に自動ビルドの出所と内容を確認し、署名を付けます。

Free code signing provided by [SignPath.io](https://about.signpath.io/), certificate by [SignPath Foundation](https://signpath.org/).

## 担当

- Authors（開発者）: [yhay81](https://github.com/yhay81)
- Reviewers（確認担当）: [yhay81](https://github.com/yhay81)
- Approvers（署名承認担当）: [yhay81](https://github.com/yhay81)

外部から提案された変更は、管理者が内容を確認してから取り込みます。署名の依頼はリリースごとに手動で承認します。

## 署名するもの

- このリポジトリのソースコードとビルド手順からGitHub Actionsで作成した公式Windows版
- 公開予定の版と同じ製品名・版番号を持つインストーラー

第三者が作成したバイナリや、手元で内容を変更したファイルには署名しません。

## プライバシー

読み上げくんが通信する相手と保存する情報は、[プライバシーについて](PRIVACY.md)に記載しています。

