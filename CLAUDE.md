# Podcast Downloader - プロジェクトガイド

## プロジェクト概要

Windows 11 向け Podcast ダウンロード専用デスクトップアプリ。macOS で開発し、Windows バイナリは GitHub Actions で生成する。

## 技術スタック

- **アプリフレームワーク**: Tauri v2
- **バックエンド**: Rust
- **フロントエンド**: React + TypeScript (Vite)
- **データベース**: SQLite (rusqlite)
- **パッケージマネージャー**: pnpm
- **CI/CD**: GitHub Actions

## 設計ドキュメント

| ドキュメント                                           | 内容                                                                                       |
| ------------------------------------------------------ | ------------------------------------------------------------------------------------------ |
| [docs/data-design.md](docs/data-design.md)             | データ設計書（ER図・テーブル定義・新着判定ロジック）                                       |
| [docs/development-guide.md](docs/development-guide.md) | 開発ガイド（環境構築・コマンド・CI/CD）                                                    |
| [docs/decisions/README.md](docs/decisions/README.md)   | 設計判断記録の一覧（ADR形式。まずこの一覧を参照し、必要な ADR のみ個別ファイルを読むこと） |

## 開発ルール

- セッション開始時に [TODO.md](TODO.md) を確認し、次に着手すべきタスクを把握すること
- タスク完了時は TODO.md のチェックボックスを更新すること
- 設計ドキュメント（data-design, development-guide, ADR）の内容と矛盾する実装をしないこと。設計を変えるなら先にドキュメントを更新すること
- git コマンドはリポジトリルートで実行すること。`-C <パス>` オプションは使わない（例: `git diff`、`git log --oneline -5`）

## 設計変更時のワークフロー

設計判断が発生した場合は、以下の順序で作業すること：

1. **ADR を作成する（必要な場合のみ）** — 粒度基準は `docs/decisions/README.md` を参照。git commit body で済むものは ADR にしない
2. **関連ドキュメントを更新する** — 下記の更新対象テーブルを参照

### 更新対象テーブル

| 変更対象               | 更新するドキュメント          |
| ---------------------- | ----------------------------- |
| テーブル・カラム定義   | data-design                   |
| マイグレーション追加   | data-design                   |
| 新着判定ロジック変更   | data-design                   |
| CI/CD・ビルド設定      | development-guide             |
| 開発コマンド・環境変更 | development-guide             |
| 重要な設計判断         | ADR（粒度基準に該当する場合） |
