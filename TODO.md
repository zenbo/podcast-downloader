# TODO

## タスク一覧

### バグ調査

- [ ] **#24 一括DLがまれに応答しない問題を調査する**
  - 現象: 個別DL直後に「新着を一括DL」を押しても反応がないことがある。画面遷移で復帰する。タイミング依存で再現性が低い
  - 仮説: 個別DL完了後の `queryClient.invalidateQueries` が `list_episodes` invoke を発火し、`std::sync::Mutex` を保持中に `batch_download_new` が同じ Mutex を取得しようとして async ランタイムが詰まる
  - 対応状況: バックエンド側の診断ログ（`log::info!`）は残存。フロントエンド側の `console.log` は削除済み
  - 次回発生時の切り分け: バックエンドログで `batch_download_new` の開始・完了を確認する
  - 影響範囲:
    - src-tauri/src/commands/download.rs: `batch_download_new` にログ追加

### 機能改善

- [x] **#25 バッチDL中に同一エピソードの単体DLを防止する**
  - 現象: バッチDL中に対象エピソードの単体DLボタンを押すと二重ダウンロードになる可能性がある
  - バックエンドの `batch_download_new` は対象エピソードを DB から動的に決定するため、フロントエンドは事前に対象を知れない
  - 方針案: バックエンドからDL開始前に対象エピソード ID リストを通知し、フロントエンドで単体DLボタンを無効化する
  - 影響範囲:
    - src-tauri/src/commands/download.rs: `batch_download_new`, `collect_new_episodes`
    - src/stores/download-context.tsx: `DownloadProvider`
