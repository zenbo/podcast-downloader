# TODO

## タスク一覧

### CI/CD

- [x] **#10 CI/CD パイプラインを構築する**
  - .github/workflows/ci.yml: PR/push → main
    - Rust lint (cargo clippy, rustfmt)
    - TypeScript lint (ESLint, Prettier)
    - テスト (cargo test, pnpm test)
    - ビルドチェック (pnpm tauri build)
  - .github/workflows/release.yml: タグ push (v*)
    - Windows バイナリビルド
    - インストーラー生成
    - GitHub Releases にドラフト作成
  - 参照: 05-development-guide

### UI 修正

- [x] **#21 番組カードのレイアウトを修正する**
  - 経緯: 設計では `□ 🎨 タイトル / 著者名  🔴 🗑` の横並びレイアウトだが、実際の画面ではカード内が縦積みになっており、チェックボックスも視認できない状態（app.png で確認）
  - 修正内容:
    - PodcastCard.tsx のレイアウトを設計通りの横並び（チェックボックス → アートワーク → タイトル/著者名 → バッジ/削除ボタン）に修正する
    - 選択時にカード全体の背景色やボーダー色を変えて、選択状態を視覚的に明示する
  - 影響範囲:
    - src/components/podcast/PodcastCard.tsx

### 設定

- [x] **#22 アプリ identifier を変更する**
  - 経緯: 現在の `com.podcast-downloader.app` は逆ドメイン表記だが、`podcast-downloader.com` を所有していない。個人ドメイン `zenbo.jp` も候補だが将来手放す可能性がある
  - 変更内容: `com.podcast-downloader.app` → `com.github.zenbo.podcast-downloader`（GitHub アカウントは失効リスクがないため）
  - 影響範囲:
    - src-tauri/tauri.conf.json: `identifier` フィールド
    - 既存の設定ファイル・DB の保存先ディレクトリ名が変わるため、既存データは引き継がれない点に注意（開発段階のため問題なし）

### UI 改善

- [x] **#23 エピソード新着マークをバックエンド判定に基づいて表示する**
  - 経緯: #20 でセクション分けを廃止した際、未DL=新着（青丸）という不正確なマーク表示も削除した。しかし新着マーク自体はあった方がよい
  - 方針:
    - `list_episodes` コマンドで `get_new_episodes` の結果と照合し、Episode に `is_new` フラグを付与する
    - フロントエンドの Episode 型に `isNew: boolean` を追加
    - EpisodeCard で `isNew` が true の場合に青丸マークを表示する
  - 影響範囲:
    - src-tauri/src/models/episode.rs: Episode に `is_new` フィールド追加
    - src-tauri/src/db/episode.rs: `row_to_episode` でデフォルト false
    - src-tauri/src/commands/episode.rs: `list_episodes` で新着フラグをセット
    - src/types/episode.ts: `isNew` 追加
    - src/components/episode/EpisodeCard.tsx: 新着マーク表示復活

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
