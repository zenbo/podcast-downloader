# ADR-013: duration フィールドの削除

## ステータス

採用

## コンテキスト

#14 の RSS テスト実装時に、feed-rs ライブラリが `itunes:duration` の `MM:SS` 形式を正しくパースしないことが判明した。

- `02:05`（2分5秒）が 2 秒として解釈される
- `HH:MM:SS` 形式や秒数指定（`125`）は正しくパースされる
- 実際の Podcast フィードでは `MM:SS` 形式が多用されている

不正確な再生時間を表示することはユーザー体験を損なうため、feed-rs の修正を待つよりもフィールドごと削除する方が適切と判断した。

## 判断

episodes テーブルから `duration` カラムを削除し、RSS パース時の duration 抽出ロジックも除去する。

理由:

1. feed-rs の `MM:SS` パースバグは上流の問題であり、このアプリ側で回避するには独自パーサーが必要になりコストが見合わない
2. duration は表示のみに使われており、ダウンロードや新着判定などのコア機能には影響しない
3. 将来 feed-rs が修正された場合、フィールドを再追加するマイグレーションは容易

## 影響範囲

- `src-tauri/src/models/episode.rs` — `Episode`, `NewEpisode` から `duration` 削除
- `src-tauri/migrations/002_drop_duration.sql` — `ALTER TABLE DROP COLUMN`
- `src-tauri/src/db/mod.rs` — マイグレーション追加
- `src-tauri/src/db/episode.rs` — INSERT / SELECT の duration 除去
- `src-tauri/src/services/rss.rs` — duration 抽出ロジックとテスト削除
- `src-tauri/src/commands/test_helpers.rs` — テストヘルパー更新
- `src/types/episode.ts` — TypeScript 型から削除
- `src/components/episode/EpisodeCard.tsx` — duration 表示削除
- テストファイル — mock データ更新
- `docs/03-data-design.md` — ER 図、テーブル定義、SQL 更新
