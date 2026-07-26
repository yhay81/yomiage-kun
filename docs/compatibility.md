# 対応環境

## 読み上げくん

| パソコン | 対応 |
|---|---|
| Windows | Windows 11、64ビット |
| Appleシリコン搭載Mac | macOS 13 Ventura以降 |
| Intel搭載Mac | macOS 13 Ventura以降 |

Windows版はMicrosoft Edge WebView2を使用します。通常はインストーラが自動で準備します。

## 声のアプリ

声のアプリ側にも対応環境があります。2026年7月時点の公式案内は次のとおりです。

| 声のアプリ | Windows | Mac | 補足 |
|---|---|---|---|
| [AivisSpeech](https://aivis-project.com/AivisSpeech) | Windows 10 22H2・Windows 11 | macOS 13以降 | 1.5GB以上の空きメモリが必要。Appleシリコン推奨 |
| [VOICEVOX](https://voicevox.hiroshiba.jp/qa/) | Windows 10・Windows 11 | macOS 14以降 | MacではVOICEVOX側の条件が優先 |

音声合成ソフトの条件は更新されることがあります。インストール前に各公式ページも確認してください。

## 確認している組み合わせ

リリース前に次を確認します。

- Windows 11 x64 + AivisSpeech
- Windows 11 x64 + VOICEVOX
- macOS Appleシリコン + AivisSpeech
- macOS Appleシリコン + VOICEVOX
- macOS Intel版はビルドと起動を確認し、AivisSpeech側が積極的な検証対象外であることを明記
