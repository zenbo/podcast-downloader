# Podcast Downloader

Podcast の音声ファイルをローカルにダウンロードし、オフライン環境で任意のプレイヤーで再生するためのデスクトップアプリケーション。

## 主な機能

- **番組登録** — Apple Podcasts の URL から番組を登録（iTunes Lookup API 経由で RSS フィードを自動取得）
- **エピソード管理** — エピソード一覧の表示、個別・一括ダウンロード、進捗表示
- **新着チェック** — RSS フィードの再取得による新着エピソードの検出と一括ダウンロード
- **ファイル管理** — ダウンロード先フォルダの設定、番組別サブフォルダの自動作成、ファイル名の文字置換ルール

## 技術スタック

| カテゴリ               | 技術                              |
| ---------------------- | --------------------------------- |
| アプリフレームワーク   | [Tauri v2](https://v2.tauri.app/) |
| バックエンド           | Rust                              |
| フロントエンド         | React + TypeScript                |
| ビルドツール           | Vite                              |
| UI                     | Tailwind CSS + shadcn/ui          |
| データベース           | SQLite (rusqlite)                 |
| パッケージマネージャー | pnpm                              |
| CI/CD                  | GitHub Actions                    |

## 対象プラットフォーム

- **実行環境**: Windows 11
- **開発環境**: macOS

## 開発環境のセットアップ

### 前提条件

- [Homebrew](https://brew.sh/)
- [mise](https://mise.jdx.dev/) (`brew install mise`)
- [Rust](https://rustup.rs/) (rustup)

### セットアップ

```bash
# リポジトリのクローン
git clone https://github.com/zenbo/podcast-downloader.git
cd podcast-downloader

# Node.js のインストール（mise が .mise.toml を参照）
mise install

# フロントエンド依存関係のインストール
pnpm install

# 開発サーバー起動
pnpm tauri dev
```

## 開発コマンド

```bash
# 開発
pnpm tauri dev          # 開発サーバー起動（ホットリロード対応）
pnpm tauri build        # プロダクションビルド

# テスト
cargo test              # Rust バックエンド
pnpm test               # フロントエンド

# Lint・フォーマット
cargo clippy -- -D warnings && cargo fmt --check   # Rust
pnpm lint && pnpm format:check                     # TypeScript/React
```

## プロジェクト構成

```
podcast-downloader/
├── src/                  # フロントエンド（React + TypeScript）
├── src-tauri/            # バックエンド（Rust + Tauri）
├── docs/                 # 設計ドキュメント
│   ├── 01-requirements.md
│   ├── 02-architecture.md
│   ├── 03-data-design.md
│   ├── 04-ui-design.md
│   ├── 05-development-guide.md
│   └── decisions/        # ADR（設計判断記録）
└── .github/workflows/    # CI/CD
    ├── ci.yml            # Lint・テスト・ビルドチェック
    └── release.yml       # Windows ビルド・リリース
```

## リリース

タグを push すると GitHub Actions が Windows 向けインストーラーを自動生成し、GitHub Releases に添付する。

```bash
git tag v0.1.0
git push origin main --tags
```

詳細は [docs/05-development-guide.md](docs/05-development-guide.md) を参照。
