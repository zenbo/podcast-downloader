# 設計判断記録 (Decision Log)

## 運用方法

- 新しい設計判断が発生した場合は、`docs/decisions/adr-XXX-<slug>.md` ファイルを作成し、本一覧にも追記する
- 各判断には ID、日付、ステータスを付与する
- ステータス: **採用** / **却下** / **保留**
- Claude Code のセッション開始時は、まず本ファイル（一覧）を参照してコンテキストを引き継ぐこと
- 詳細が必要な ADR のみ個別ファイルを参照すること（コンテキストウィンドウ節約のため）

## 判断一覧

| ID | 日付 | タイトル | ステータス |
|----|------|---------|-----------|
| [ADR-001](adr-001-app-framework.md) | 2026-02-21 | アプリケーションフレームワークの選定 → Tauri v2 | 採用 |
| [ADR-002](adr-002-frontend-framework.md) | 2026-02-21 | フロントエンドフレームワークの選定 → React + TypeScript (Vite) | 採用 |
| [ADR-003](adr-003-database.md) | 2026-02-21 | データベースの選定 → SQLite (rusqlite) | 採用 |
| [ADR-004](adr-004-apple-podcasts-rss.md) | 2026-02-21 | Apple Podcasts URL からの RSS 取得方式 → iTunes Lookup API | 採用 |
| [ADR-005](adr-005-new-episode-detection.md) | 2026-02-21 | 新着エピソードの判定ロジック → 最終DLエピソードの配信日基準 | 採用 |
| [ADR-006](adr-006-package-manager.md) | 2026-02-21 | パッケージマネージャーの選定 → pnpm | 採用 |
| [ADR-007](adr-007-windows-build.md) | 2026-02-21 | Windows ビルドの方式 → GitHub Actions のみ | 採用 |
| [ADR-008](adr-008-frontend-state-management.md) | 2026-02-22 | フロントエンド状態管理の選定 → TanStack Query + useState | 採用 |
| [ADR-009](adr-009-frontend-libraries.md) | 2026-02-22 | フロントエンドライブラリの選定 → React Router / Tailwind CSS / shadcn/ui | 採用 |
| [ADR-010](adr-010-data-simplification.md) | 2026-02-22 | データ保存方式の簡素化 → SQLite 2テーブル + JSON設定 | 採用 |
| [ADR-011](adr-011-adr-file-splitting.md) | 2026-02-22 | ADR のファイル分割 → ディレクトリ分割方式 | 採用 |
| [ADR-012](adr-012-commands-testability.md) | 2026-02-23 | Commands 層のテスタビリティ改善 → トレイト抽象化 + コールバック汎用化 | 採用 |
