# TODO

## 依存関係

```
#1 データモデル (models/)           ← 最初に着手
  ├─ #2 エラー型 (error.rs)
  │   └─ #3 DB 層 (db/)
  │       └─ #4 サービス層 (services/)
  │           └─ #5 Tauri コマンド (commands/)
  │               ├─ #7 番組一覧画面
  │               ├─ #8 エピソード一覧画面
  │               └─ #9 設定画面
  └─ #6 フロントエンド Services 層・型定義
      ├─ #7 番組一覧画面
      ├─ #8 エピソード一覧画面
      └─ #9 設定画面

#10 CI/CD パイプライン              ← 独立して着手可能

#11 フロントエンドテスト基盤         ← 独立して着手可能
  ├─ #12 services/ ユニットテスト
  └─ #13 hooks/ ユニットテスト

#14 RSS解析層テスト                 ← 独立して着手可能
#15 Commands層リファクタリング       ← 独立して着手可能
  └─ #16 Commands層テスト
#17 duration フィールド削除          ← 独立して着手可能
```

## タスク一覧

### Rust バックエンド

- [x] **#1 Rust データモデル (models/) を実装する**
  - models/podcast.rs: Podcast, PodcastSummary, PodcastNewCount
  - models/episode.rs: Episode
  - models/settings.rs: AppSettings, DownloadProgress, BatchDownloadProgress, CharacterReplacement
  - すべて serde の Serialize/Deserialize を derive する
  - 参照: 02-architecture 4.4, 05-development-guide

- [x] **#2 エラー型 (error.rs) を完成させる** (blocked by: #1)
  - Database(rusqlite::Error), Http(reqwest::Error), RssParse(String), InvalidUrl(String), PodcastIdNotFound(String), FeedUrlNotFound, FileSystem(std::io::Error), Other(String)
  - Serialize 実装と thiserror の #[error] アトリビュートを設定
  - 参照: 02-architecture 6.1

- [x] **#3 DB 層 (db/) を実装する** (blocked by: #1, #2)
  - db/mod.rs: DB 接続管理、マイグレーション実行 (rusqlite_migration 2.x API)、PRAGMA foreign_keys = ON
  - db/podcast.rs: insert, list (with new_episode_count), get, delete, update_last_checked
  - db/episode.rs: insert_bulk, list_by_podcast, get, mark_downloaded, 新着判定クエリ
  - アプリ起動時に app_data_dir に DB ファイルを作成・接続する初期化処理を含む
  - 参照: 02-architecture 3.2, 03-data-design

- [x] **#4 サービス層 (services/) を実装する** (blocked by: #1, #2, #3)
  - services/apple_podcasts.rs: URL から Podcast ID 抽出 (正規表現)、iTunes Lookup API で feedUrl 取得
  - services/rss.rs: RSS フィード取得・パース (feed-rs)、PodcastFeed 構造体への変換
  - services/downloader.rs: HTTP ストリーミングダウンロード、Channel API で進捗通知、ファイル保存
  - services/filename.rs: 文字置換ルール適用、OS 禁止文字サニタイズ
  - 参照: 02-architecture 3.4-3.7

- [x] **#5 Tauri コマンド (commands/) を実装する** (blocked by: #3, #4)
  - commands/podcast.rs: register_podcast, list_podcasts, delete_podcast
  - commands/episode.rs: list_episodes, check_new_episodes, check_all_new
  - commands/download.rs: download_episode (Channel API), batch_download_new (Channel API)
  - commands/settings.rs: get_settings, update_settings, select_folder (tauri-plugin-dialog)
  - lib.rs に全コマンドを .invoke_handler() で登録、setup() で DB 初期化と設定ファイル初期化
  - 参照: 02-architecture 3.3

### フロントエンド

- [x] **#6 フロントエンド Services 層と型定義を実装する** (blocked by: #1)
  - src/types/podcast.ts: Podcast, PodcastSummary, PodcastNewCount
  - src/types/episode.ts: Episode
  - src/types/settings.ts: AppSettings, DownloadProgress, BatchDownloadProgress
  - src/services/podcast.ts: registerPodcast, listPodcasts, deletePodcast
  - src/services/episode.ts: listEpisodes, checkNewEpisodes, checkAllNew
  - src/services/download.ts: downloadEpisode, batchDownloadNew (Channel API 対応)
  - src/services/settings.ts: getSettings, updateSettings, selectFolder
  - 参照: 02-architecture 4.3-4.4

- [x] **#7 番組一覧画面 (PodcastListPage) を実装する** (blocked by: #5, #6)
  - 番組一覧のカード/リスト表示（アートワーク、タイトル、新着バッジ）
  - 番組登録ダイアログ（Apple Podcasts URL 入力）
  - 番組削除の確認ダイアログ
  - 「全番組の新着チェック」ボタン
  - 「選択番組の新着一括DL」機能
  - 番組クリックで /podcast/:id に遷移
  - 参照: 04-ui-design

- [x] **#8 エピソード一覧画面 (EpisodeListPage) を実装する** (blocked by: #5, #6)
  - エピソード一覧表示（タイトル、配信日、DL 状態）
  - 個別エピソードのダウンロードボタン（進捗バー付き）
  - 新着エピソードの一括ダウンロード
  - 番組一覧への戻りナビゲーション
  - 参照: 04-ui-design

- [x] **#9 設定画面 (SettingsPage) を実装する** (blocked by: #5, #6)
  - ダウンロード先フォルダの選択（フォルダ選択ダイアログ連携）
  - 文字置換ルールの一覧・追加・削除・順序変更
  - フォールバック置換文字の設定
  - tauri-plugin-store 経由での設定保存・読み込み
  - 参照: 04-ui-design, 03-data-design 4.1

### フロントエンドテスト

- [x] **#11 フロントエンドテスト基盤をセットアップする**
  - vitest.config.ts を作成（jsdom 環境設定）
  - Tauri invoke のグローバルモックを setup ファイルに定義
  - 参照: 05-development-guide 3.2

- [x] **#12 services/ 層のユニットテストを実装する** (blocked by: #11)
  - src/services/podcast.ts: invoke の呼び出し引数・戻り値の変換を検証
  - src/services/episode.ts: 同上
  - src/services/download.ts: Channel API 対応のイベントリスナー登録を検証
  - src/services/settings.ts: 同上

- [x] **#13 hooks/ のユニットテストを実装する** (blocked by: #11)
  - use-podcasts: 番組一覧取得・登録・削除の状態遷移とエラーハンドリング
  - use-episodes: エピソード取得・新着チェックの状態遷移
  - use-settings: 設定読み込み・保存の状態遷移

### Rust バックエンドテスト

- [x] **#14 RSS 解析層のユニットテストを実装する**
  - services/rss.rs のパース処理に対するテスト
  - フィクスチャ XML（正常系・異常系）を用意してネットワーク不要でテスト
  - テスト観点:
    - 標準的な RSS 2.0 フィードのパース
    - タグ欠落・日付フォーマット揺れなど実際のフィードで起こりうる異常への耐性
    - エピソード数が 0 件のフィード
    - audio_url が存在しないエントリのスキップ

- [x] **#15 Commands 層をテスト可能にリファクタリングする**
  - batch_download_new のインライン HTTP ダウンロード実装を downloader::download() と共通化し、コード重複を解消する
  - 外部 IO（HTTP クライアント、ファイルシステム、Tauri Store）をトレイトで抽象化し、テスト時にモックを差し込めるようにする
  - check_all_new の複数回 db::podcast::get() 呼び出しを 1 クエリに最適化する

- [x] **#16 Commands 層のユニットテストを実装する** (blocked by: #15)
  - download_episode: 設定未設定時のエラー、DL 失敗時に mark_downloaded が呼ばれないこと
  - batch_download_new: 一部エピソード失敗時の部分的成功、進捗通知の completed_count の正確性
  - check_all_new: 1 番組の RSS 取得失敗時に他の番組は継続すること
  - register_podcast: RSS パースエラー時に DB 登録が行われないこと

### 設計変更

- [x] **#17 duration フィールドを削除する**
  - 経緯: #14 の RSS テスト実装時に、feed-rs が `itunes:duration` の `MM:SS` 形式を正しくパースしないことが判明（`02:05` が 2 秒として解釈される）。`HH:MM:SS` や秒数指定は正常。実際の Podcast フィードでは `MM:SS` 形式が多用されており、不正確な値を表示するくらいなら削除する方が良いと判断
  - ADR を作成する
  - 影響範囲:
    - Models: NewEpisode, Episode から duration フィールド削除
    - DB: episodes テーブルの duration カラム削除（マイグレーション）
    - services/rss.rs: duration 抽出ロジック削除
    - フロントエンド: duration の表示箇所を削除
    - 設計ドキュメント: 03-data-design 等を更新

### CI/CD

- [ ] **#10 CI/CD パイプラインを構築する**
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
