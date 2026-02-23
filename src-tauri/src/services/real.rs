use std::path::Path;

use async_trait::async_trait;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use crate::error::AppError;
use crate::models::episode::DownloadProgress;
use crate::models::podcast::PodcastFeed;
use crate::models::settings::AppSettings;
use crate::services::traits::*;
use crate::services::{apple_podcasts, downloader, rss};

const STORE_FILENAME: &str = "settings.json";
const SETTINGS_KEY: &str = "settings";

/// 本番用の RSS フィード取得
pub struct RealRssFetcher;

#[async_trait]
impl RssFetcher for RealRssFetcher {
    async fn fetch_and_parse(&self, feed_url: &str) -> Result<PodcastFeed, AppError> {
        rss::fetch_and_parse(feed_url).await
    }
}

/// 本番用の Apple Podcasts URL 解決
pub struct RealFeedUrlResolver;

#[async_trait]
impl FeedUrlResolver for RealFeedUrlResolver {
    async fn resolve_feed_url(&self, url: &str) -> Result<String, AppError> {
        apple_podcasts::resolve_feed_url(url).await
    }
}

/// 本番用のファイルダウンローダー
pub struct RealFileDownloader;

#[async_trait]
impl FileDownloader for RealFileDownloader {
    async fn download(
        &self,
        audio_url: &str,
        save_path: &Path,
        episode_id: i64,
        on_progress: Box<dyn FnMut(DownloadProgress) + Send>,
    ) -> Result<(), AppError> {
        downloader::download(audio_url, save_path, episode_id, on_progress).await
    }
}

/// 本番用の設定ストア（Tauri Store を使用）
pub struct TauriSettingsStore {
    app_handle: AppHandle,
}

impl TauriSettingsStore {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }
}

impl SettingsStore for TauriSettingsStore {
    fn load_settings(&self) -> Result<AppSettings, AppError> {
        let store = self
            .app_handle
            .store(STORE_FILENAME)
            .map_err(|e| AppError::Other(e.to_string()))?;

        let settings = store
            .get(SETTINGS_KEY)
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default();

        Ok(settings)
    }

    fn save_settings(&self, settings: &AppSettings) -> Result<(), AppError> {
        let store = self
            .app_handle
            .store(STORE_FILENAME)
            .map_err(|e| AppError::Other(e.to_string()))?;

        store.set(
            SETTINGS_KEY,
            serde_json::to_value(settings).map_err(|e| AppError::Other(e.to_string()))?,
        );

        Ok(())
    }
}
