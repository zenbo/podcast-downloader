# ADR-012: Commands 層のテスタビリティ改善

## ステータス

採用

## コンテキスト

Commands 層（Tauri IPC エントリポイント）に以下の問題がある:

1. **コード重複**: `batch_download_new` が `downloader::download()` と同じ HTTP ダウンロードコードを約 40 行重複して持っている。`Channel<DownloadProgress>` と `Channel<BatchDownloadProgress>` の型不一致が原因
2. **テスト不可能**: 外部 IO（HTTP クライアント、ファイルシステム、Tauri Store）が commands 内で直接呼ばれており、ユニットテスト時にモック差し込みが不可能
3. **N+1 クエリ**: `check_all_new` が `list()` で全番組のサマリーを取得した後、ループ内で `get()` を N 回呼んでいる

## 判断

### 1. downloader のコールバック設計

`downloader::download()` のシグネチャを `&Channel<DownloadProgress>` から `impl FnMut(DownloadProgress)` に変更する。

- 呼び出し側がクロージャで進捗通知の方法を自由に決められる
- `download_episode` は `Channel<DownloadProgress>` に直接送信
- `batch_download_new` は `BatchDownloadProgress` にラップして送信

### 2. トレイト抽象化のスコープと設計

**細粒度トレイト（インターフェース分離原則）** を採用する:

- `RssFetcher` — RSS フィード取得・パース
- `FeedUrlResolver` — Apple Podcasts URL → feed URL 解決
- `FileDownloader` — 音声ファイル HTTP ダウンロード
- `SettingsStore` — アプリ設定の読み書き

1 つの大きな `ExternalServices` トレイトにまとめる案は却下。各コマンドが必要なトレイトのみに依存するほうが、テスト時のモック実装が小さく済む。

DB 層はトレイト化しない（インメモリ SQLite でテスト可能なため）。
ダイアログ UI もトレイト化しない（ユニットテスト対象外）。

### 3. async trait の実装方法

`async-trait` クレートを使用する。

Rust 1.75 以降 `async fn` in traits は安定だが、`dyn Trait` での動的ディスパッチにはまだ制限がある。Tauri の `State<T>` に `Arc<dyn Trait>` を入れる必要があるため、`async_trait` を採用する。

### 4. サービスコンテナの設計

`ServiceContainer` 構造体に全トレイトオブジェクトの `Arc<dyn Trait>` をまとめ、Tauri の `State<ServiceContainer>` として注入する。

### 5. コマンドのテストパターン

「薄いコマンド + `_impl` 関数」パターンを採用する:

- `#[tauri::command]` 関数は `State<T>` からの取り出しだけを行う
- 実際のビジネスロジックは `_impl` 関数に委譲する
- テスト時は `_impl` 関数を直接呼び出し、`State<T>` のボイラープレートを回避する

### 6. check_all_new の最適化

`db::podcast::list_all()` 関数を追加し、全番組の完全な `Podcast` レコードを 1 クエリで取得する。既存の `row_to_podcast` ヘルパーを再利用する。

## 影響範囲

- `src-tauri/src/services/downloader.rs` — シグネチャ変更
- `src-tauri/src/services/traits.rs` — 新規（トレイト定義）
- `src-tauri/src/services/real.rs` — 新規（本番実装）
- `src-tauri/src/db/podcast.rs` — `list_all()` 追加
- `src-tauri/src/commands/*.rs` — ServiceContainer 導入、\_impl パターン
- `src-tauri/src/lib.rs` — ServiceContainer の State 登録
- `src-tauri/Cargo.toml` — `async-trait` 依存追加
