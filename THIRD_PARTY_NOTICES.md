# 利用しているソフトウェア

読み上げくんはMIT Licenseで公開しています。アプリの作成には、別のライセンスで公開されているソフトウェアも利用しています。

主なものは次のとおりです。

- [Tauri](https://tauri.app/) — デスクトップアプリ
- [Serenity](https://github.com/serenity-rs/serenity) — Discordとの接続
- [Songbird](https://github.com/serenity-rs/songbird) — Discord通話での音声再生
- [Reqwest](https://github.com/seanmonstar/reqwest) — 音声合成ソフトとの通信
- [Vite](https://vite.dev/) — 画面の作成

すべての直接・間接依存関係と正確な版は[`Cargo.lock`](Cargo.lock)および[`package-lock.json`](apps/desktop/package-lock.json)に記録しています。リリース時にはCIで、配布対象で使われるRust依存関係の既知の問題、入手元、ライセンスを検査します。

AivisSpeechとVOICEVOXは読み上げくんに同梱していません。利用者が各公式サイトから別に入手します。生成した音声の利用条件は、使用する音声モデルやキャラクターの規約を確認してください。
