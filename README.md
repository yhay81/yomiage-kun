# yomiage-kun

Discord bot which can read Japanese chat and say it in voice channnel.
日本語チャットを音声読み上げしボイスに流すdiscordのbotです。

## インストール

導入がなかなか大変だと思います。
できた方いらっしゃいましたら、手順やコードの改善点のコントリビューションをお待ちしています。

discord.py を使っています。
Python 3.5以上が必要です。
CentOS7 で動作していました。

### Open JTalk
Open Jtalkというライブラリを利用して音声読み上げをしているため、こちらを導入する必要があります。
http://open-jtalk.sourceforge.net/

### 音響モデル
以下の記事が詳しいですが、音響モデルも必要です。
http://mahoro-ba.net/e1875.html
コードをそのまま使うためにはこれら音響モデルのファイルを名前を合わせて特定の位置に置いておく必要があります。

その他音声再生に関して必要なライブラリがあるかもしれません。
