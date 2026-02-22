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

作業前に以下のドキュメントを参照すること。

| ドキュメント | 内容 |
|------------|------|
| [docs/01-requirements.md](docs/01-requirements.md) | 要件定義書（機能要件・非機能要件） |
| [docs/02-architecture.md](docs/02-architecture.md) | アーキテクチャ設計書（モジュール構成・Tauriコマンド一覧・IPC設計） |
| [docs/03-data-design.md](docs/03-data-design.md) | データ設計書（ER図・テーブル定義・新着判定ロジック） |
| [docs/04-ui-design.md](docs/04-ui-design.md) | 画面設計書（画面遷移・レイアウト・操作フロー） |
| [docs/05-development-guide.md](docs/05-development-guide.md) | 開発ガイド（環境構築・コマンド・CI/CD） |
| [docs/decisions/README.md](docs/decisions/README.md) | 設計判断記録の一覧（ADR形式。まずこの一覧を参照し、必要な ADR のみ個別ファイルを読むこと） |

## 開発ルール

- 設計判断を変更・追加した場合は `docs/decisions/` に ADR ファイルを作成し、`README.md` の一覧にも追記すること
- 設計ドキュメントの内容と矛盾する実装をしないこと。設計を変えるなら先にドキュメントを更新すること
