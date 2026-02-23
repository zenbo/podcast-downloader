# TODO

## タスク一覧

### バグ調査

- [ ] **#24 一括DLがまれに応答しない問題を調査する**
  - 現象: 個別DL直後に「新着を一括DL」を押しても反応がないことがある。画面遷移で復帰する。タイミング依存で再現性が低い
  - 仮説: 個別DL完了後の `queryClient.invalidateQueries` が `list_episodes` invoke を発火し、`std::sync::Mutex` を保持中に `batch_download_new` が同じ Mutex を取得しようとして async ランタイムが詰まる
  - 対応状況: 診断ログを追加済み（フロントエンド `[BatchDL]` プレフィックス、バックエンド `log::info!`）
  - 次回発生時の切り分け:
    - `[BatchDL] 開始` が出ない → ボタンのイベント問題
    - `[BatchDL] 開始` のみで止まる → invoke のハング（Mutex 競合の可能性大）
    - `[BatchDL] 完了` も出る → バックエンド側で対象 0 件
  - 影響範囲:
    - src/pages/EpisodeListPage.tsx: `handleBatchDownload` にログ追加
    - src-tauri/src/commands/download.rs: `batch_download_new` にログ追加
