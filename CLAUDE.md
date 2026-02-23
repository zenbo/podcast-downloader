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

| ドキュメント                                                 | 内容                                                                                       |
| ------------------------------------------------------------ | ------------------------------------------------------------------------------------------ |
| [docs/01-requirements.md](docs/01-requirements.md)           | 要件定義書（機能要件・非機能要件）                                                         |
| [docs/02-architecture.md](docs/02-architecture.md)           | アーキテクチャ設計書（モジュール構成・Tauriコマンド一覧・IPC設計）                         |
| [docs/03-data-design.md](docs/03-data-design.md)             | データ設計書（ER図・テーブル定義・新着判定ロジック）                                       |
| [docs/04-ui-design.md](docs/04-ui-design.md)                 | 画面設計書（画面遷移・レイアウト・操作フロー）                                             |
| [docs/05-development-guide.md](docs/05-development-guide.md) | 開発ガイド（環境構築・コマンド・CI/CD）                                                    |
| [docs/decisions/README.md](docs/decisions/README.md)         | 設計判断記録の一覧（ADR形式。まずこの一覧を参照し、必要な ADR のみ個別ファイルを読むこと） |

## 開発ルール

- セッション開始時に [TODO.md](TODO.md) を確認し、次に着手すべきタスクを把握すること
- タスク完了時は TODO.md のチェックボックスを更新すること
- 設計ドキュメントの内容と矛盾する実装をしないこと。設計を変えるなら先にドキュメントを更新すること
- git コマンドはリポジトリルートで実行すること。`-C <パス>` オプションは使わない（例: `git diff`、`git log --oneline -5`）

## 設計変更時のワークフロー

設計判断が発生した場合は、以下の順序で作業すること：

1. **ADR を作成する** — 判断内容を `docs/decisions/adr-XXX-<slug>.md` に記録し、`README.md` の一覧にも追記する
2. **関連ドキュメントを修正する** — 下記の影響マップを参照して、関連ドキュメントに変更を反映する
3. **コミット前に grep で残留チェック** — 変更した概念・用語（テーブル名、保存先、コマンド名等）について、旧い表現が `docs/` 配下に残っていないか確認する

### ドキュメント間の影響マップ

| 変更対象                           | 影響を受けるドキュメント                                                                                                  |
| ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| テーブル・カラム定義               | 03-data-design, 02-architecture（コマンド戻り値・フロー図）, 04-ui-design（表示要素）                                     |
| Tauri コマンド（追加・変更・削除） | 02-architecture（コマンド一覧・フロー図）, 04-ui-design（操作→動作の対応）                                                |
| データ保存先（SQLite ↔ JSON）      | 01-requirements（FR-014 等）, 02-architecture（初期化フロー・クレート）, 03-data-design, 04-ui-design（保存ボタンの動作） |
| フロントエンドライブラリ           | 02-architecture（状態管理・ページ構成）, 04-ui-design（UIフレームワーク）, 05-development-guide（依存関係）               |
| CI/CD・ビルド設定                  | 02-architecture（ディレクトリツリー）, 05-development-guide（CI yml・コマンド）                                           |
| ファイル命名規則                   | 02-architecture（3.6節）, 03-data-design（設定スキーマ）                                                                  |
