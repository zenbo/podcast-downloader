# ADR-018: アプリアイコンのデザイン方針

## ステータス

採用

## 日付

2026-02-23

## コンテキスト

Tauri のデフォルトアイコンのままだったため、アプリ固有のアイコンを用意する必要があった。

## 決定

- **モチーフ**: ヘッドホン + 下矢印（ダウンロード）
- **スタイル**: フラットデザイン、ベクター風、テキストなし
- **配色**: 青〜ティール系グラデーション背景 + 白いシンボル
- **生成方法**: ChatGPT（DALL-E）で 1024x1024 の PNG を生成し、`pnpm tauri icon` で全サイズを一括生成

### 画像生成プロンプト

```
A minimal flat-design app icon, 1024x1024, square with rounded corners.
A pair of over-ear headphones in white, with a bold downward arrow centered
between the ear cups, on a gradient background from deep blue (#1E3A5F)
to teal (#2A9D8F). No text, no shadow, no 3D effects. Simple geometric
shapes, clean vector style. Suitable for a desktop application icon that
remains legible at small sizes.
```

## 理由

- ヘッドホンは Podcast（音声コンテンツ）を、下矢印はダウンロードを直感的に表現できる
- フラットデザインは小さいサイズ（32x32 等）でも視認性が高い
- プロンプトを記録しておくことで、将来のアイコン再生成・微調整が容易になる

## 影響

- `src-tauri/icons/` 配下のアイコンファイルを差し替え
