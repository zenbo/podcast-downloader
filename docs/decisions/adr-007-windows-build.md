# ADR-007: Windows ビルドの方式

| 項目       | 値         |
| ---------- | ---------- |
| 日付       | 2026-02-21 |
| ステータス | 採用       |

## コンテキスト

開発環境は macOS だが、配布ターゲットは Windows 11 である。Windows 向けバイナリの生成方法を決定する。

## 検討した選択肢

1. **GitHub Actions のみ** — Windows バイナリは CI 上の Windows ランナーで生成。macOS ではローカル macOS 版でデバッグ
2. **macOS 上でクロスコンパイル** — ローカルでも Windows 向けバイナリを生成する

## 決定

**選択肢 1: GitHub Actions のみ** で Windows バイナリを生成する。

## 理由

- Tauri のクロスコンパイルには制約が多く、特に Windows ターゲットは WebView2 依存がありローカルでのクロスビルドが困難
- `tauri-action` を使用すれば、GitHub Actions 上で簡単に Windows インストーラーを生成できる
- macOS 版で開発・デバッグし、最終的な Windows 版は CI で生成するワークフローが現実的
- Tauri はクロスプラットフォーム前提のため、macOS で動作確認したコードは基本的に Windows でも動作する

## 影響・リスク

- Windows 固有の問題（ファイルパスのバックスラッシュ等）はローカルでは検出しにくい
- CI でのビルド失敗時はフィードバックループが長くなる
