# 開発ガイド

## 1. 開発環境構築

### 1.1 前提条件

| ツール        | 用途                           | インストール方法            |
| ------------- | ------------------------------ | --------------------------- |
| Homebrew      | macOS パッケージマネージャー   | [brew.sh](https://brew.sh/) |
| mise          | ランタイムバージョン管理       | `brew install mise`         |
| Rust (rustup) | Rust ツールチェイン            | 下記参照                    |
| Node.js       | フロントエンドビルド           | mise 経由                   |
| pnpm          | Node.js パッケージマネージャー | mise 経由                   |

### 1.2 セットアップ手順

#### 1. mise のインストールと設定

```bash
brew install mise

# シェル設定に追加（zsh の場合）
echo 'eval "$(mise activate zsh)"' >> ~/.zshrc
source ~/.zshrc
```

#### 2. Rust のインストール

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# インストール確認
rustc --version
cargo --version
```

#### 3. プロジェクトのクローンと依存関係インストール

```bash
git clone <repository-url>
cd podcast-downloader

# mise が .mise.toml を読んで Node.js をインストール
mise install

# pnpm のインストール（未インストールの場合）
mise use -g pnpm

# フロントエンド依存関係のインストール
pnpm install
```

#### 4. Tauri プラグインの追加

```bash
cd src-tauri

# tauri-plugin-store: アプリケーション設定の JSON 保存
cargo add tauri-plugin-store

# tauri-plugin-dialog: フォルダ選択ダイアログ
cargo add tauri-plugin-dialog
```

`src-tauri/src/lib.rs` でプラグインを登録する:

```rust
tauri::Builder::default()
    .plugin(tauri_plugin_store::Builder::new().build())
    .plugin(tauri_plugin_dialog::init())
    // ...
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
```

#### 5. 動作確認

```bash
# 開発サーバー起動（macOS 版）
pnpm tauri dev
```

### 1.3 .mise.toml

プロジェクトルートに配置する `.mise.toml` で Node.js バージョンを固定する。

```toml
[tools]
node = "24"     # LTS バージョンを指定
```

### 1.4 .node-version

CI の `actions/setup-node` は `.mise.toml` を直接読めないため、`.node-version` ファイルもプロジェクトルートに配置する。

```
24
```

ローカル開発では mise が `.mise.toml` を参照し、CI では `actions/setup-node` が `.node-version` を参照する。

## 2. 開発コマンド一覧

### 2.1 開発・ビルド

| コマンド           | 説明                                     |
| ------------------ | ---------------------------------------- |
| `pnpm tauri dev`   | 開発サーバー起動（ホットリロード対応）   |
| `pnpm tauri build` | プロダクションビルド（ローカル OS 向け） |
| `pnpm dev`         | フロントエンドのみの開発サーバー起動     |
| `pnpm build`       | フロントエンドのみのビルド               |

### 2.2 テスト

| コマンド                    | 説明                                       |
| --------------------------- | ------------------------------------------ |
| `cargo test`                | Rust バックエンドの全テスト実行            |
| `cargo test -- --nocapture` | テスト出力を表示しながら実行               |
| `pnpm test`                 | フロントエンドの全テスト実行               |
| `pnpm test:watch`           | フロントエンドテストをウォッチモードで実行 |

### 2.3 Lint・フォーマット

| コマンド                      | 説明                              |
| ----------------------------- | --------------------------------- |
| `cargo clippy`                | Rust の静的解析                   |
| `cargo clippy -- -D warnings` | 警告をエラーとして扱う（CI 相当） |
| `cargo fmt --check`           | Rust のフォーマットチェック       |
| `cargo fmt`                   | Rust のフォーマット実行           |
| `pnpm lint`                   | TypeScript/React の ESLint 実行   |
| `pnpm lint:fix`               | ESLint の自動修正                 |
| `pnpm format:check`           | Prettier のフォーマットチェック   |
| `pnpm format`                 | Prettier のフォーマット実行       |

### 2.4 ローカルで CI 相当を実行

CI と同じ検証をローカルで実行するためのコマンド:

```bash
# Lint（Rust + TypeScript）
cargo clippy -- -D warnings && cargo fmt --check && pnpm lint && pnpm format:check

# テスト（Rust + TypeScript）
cargo test && pnpm test

# ビルド確認
pnpm tauri build
```

## 3. テスト方針

### 3.1 Rust バックエンドテスト

- 各モジュール内に `#[cfg(test)]` で定義
- 外部 API（iTunes、RSS、ファイルダウンロード）は **trait によるモック化**（ADR-012）で分離済み。テスト用モックは `src-tauri/src/commands/test_helpers.rs` に集約
- DB テストは **インメモリ SQLite** (`":memory:"`) を使用し、テストごとにマイグレーションを実行

### 3.2 フロントエンドテスト

- **Vitest** をテストランナーとして使用（Vite との親和性が高い）
- Tauri の `invoke` は `vi.mock` でモック化

## 4. Lint・フォーマット設定

### 4.1 Rust

- **clippy**: Rust 標準の Linter。`cargo clippy` で実行
- **rustfmt**: Rust 標準のフォーマッター。`cargo fmt` で実行
- 設定ファイル: `rustfmt.toml`（必要に応じてカスタマイズ）

### 4.2 TypeScript / React

- **ESLint**: JavaScript/TypeScript の静的解析
- **Prettier**: コードフォーマッター
- 設定ファイル: `eslint.config.js`（フラットコンフィグ）/ `.prettierrc`

## 5. CI/CD 設計

### 5.1 パイプライン概要

```mermaid
flowchart LR
    subgraph CI["ci.yml (PR / push)"]
        direction TB
        Lint["Lint<br>clippy + ESLint<br>fmt + Prettier"]
        Test["Test<br>cargo test<br>pnpm test"]
        Build["Build Check<br>pnpm tauri build"]
        Lint --> Test --> Build
    end

    subgraph Release["release.yml (tag push)"]
        direction TB
        WinBuild["Windows Build<br>tauri-action"]
        GHRelease["GitHub Release<br>インストーラー添付"]
        WinBuild --> GHRelease
    end

    Push([push / PR]) --> CI
    Tag([tag push]) --> Release
```

### 5.2 ci.yml — Lint・テスト・ビルドチェック

- **トリガー**: プルリクエスト、main ブランチへの push
- **実行環境**: ubuntu-latest（Lint・テストは Linux で十分）
- **ステップ**: Rust セットアップ → システム依存パッケージ → Node.js/pnpm → clippy + fmt → ESLint + Prettier → cargo test + pnpm test

### 5.3 release.yml — ビルド・リリース

- **トリガー**: `v*` パターンのタグ push
- **実行環境**: windows-latest（ターゲットプラットフォーム）
- **ビルド**: `tauri-apps/tauri-action` でビルドし、GitHub Releases にドラフトリリースとして添付

ワークフロー定義の詳細は `.github/workflows/` 配下の各ファイルを参照。

## 6. リリース手順

### 6.1 バージョニング方針

Semantic Versioning (SemVer) に従う: `MAJOR.MINOR.PATCH`

- **MAJOR**: 互換性のない変更
- **MINOR**: 後方互換性のある機能追加
- **PATCH**: 後方互換性のあるバグ修正

### 6.2 リリース手順

1. `src-tauri/tauri.conf.json` の `version` を更新する
2. `package.json` の `version` を更新する（一致させる）
3. 変更をコミットする
4. バージョンタグを作成して push する

```bash
# バージョン更新をコミット
git add -A
git commit -m "chore: bump version to v0.1.0"

# タグを作成して push
git tag v0.1.0
git push origin main --tags
```

5. GitHub Actions の release.yml が自動実行される
6. GitHub Releases にドラフトリリースが作成される
7. ドラフトの内容を確認し、公開する

## 7. トラブルシューティング

### Tauri の開発サーバーが起動しない

- Rust ツールチェインが正しくインストールされているか確認: `rustc --version`
- macOS の場合、Xcode Command Line Tools が必要: `xcode-select --install`
- `src-tauri/` ディレクトリで `cargo build` が通るか個別に確認

### pnpm install でエラーが発生する

- Node.js のバージョンが `.mise.toml` で指定したバージョンと一致しているか確認: `node --version`
- `node_modules` を削除して再インストール: `rm -rf node_modules && pnpm install`

### CI でビルドが失敗する

- ローカルで CI 相当のコマンドを実行して再現を試みる（2.4 節参照）
- Rust の依存クレートのバージョン互換性を確認
- `pnpm-lock.yaml` が最新かどうか確認
