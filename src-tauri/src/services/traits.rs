use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;

use crate::error::AppError;
use crate::models::episode::DownloadProgress;
use crate::models::podcast::PodcastFeed;
use crate::models::settings::AppSettings;

/// RSS フィード取得・パース
#[async_trait]
pub trait RssFetcher: Send + Sync {
    async fn fetch_and_parse(&self, feed_url: &str) -> Result<PodcastFeed, AppError>;
}

/// Apple Podcasts URL → RSS フィード URL 解決
#[async_trait]
pub trait FeedUrlResolver: Send + Sync {
    async fn resolve_feed_url(&self, apple_podcasts_url: &str) -> Result<String, AppError>;
}

/// 音声ファイル HTTP ダウンロード
#[async_trait]
pub trait FileDownloader: Send + Sync {
    async fn download(
        &self,
        audio_url: &str,
        save_path: &Path,
        episode_id: i64,
        on_progress: Box<dyn FnMut(DownloadProgress) + Send>,
    ) -> Result<(), AppError>;
}

/// アプリ設定の読み書き
pub trait SettingsStore: Send + Sync {
    fn load_settings(&self) -> Result<AppSettings, AppError>;
    fn save_settings(&self, settings: &AppSettings) -> Result<(), AppError>;
}

/// Commands 層が依存する外部サービスのコンテナ
pub struct ServiceContainer {
    pub rss_fetcher: Arc<dyn RssFetcher>,
    pub feed_url_resolver: Arc<dyn FeedUrlResolver>,
    pub file_downloader: Arc<dyn FileDownloader>,
    pub settings_store: Arc<dyn SettingsStore>,
}
