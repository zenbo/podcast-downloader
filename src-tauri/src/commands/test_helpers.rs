use std::path::Path;
use std::sync::Mutex;

use async_trait::async_trait;

use crate::db::{self, DbState};
use crate::error::AppError;
use crate::models::episode::{DownloadProgress, NewEpisode};
use crate::models::podcast::PodcastFeed;
use crate::models::settings::AppSettings;
use crate::services::traits::*;

// ---------------------------------------------------------------------------
// MockRssFetcher
// ---------------------------------------------------------------------------

pub struct MockRssFetcher {
    responses: Mutex<Vec<Result<PodcastFeed, String>>>,
}

impl MockRssFetcher {
    pub fn new(responses: Vec<Result<PodcastFeed, String>>) -> Self {
        Self {
            responses: Mutex::new(responses),
        }
    }

    pub fn always_ok(feed: PodcastFeed) -> Self {
        let responses = (0..100).map(|_| Ok(feed.clone())).collect();
        Self::new(responses)
    }

    pub fn always_err(msg: &str) -> Self {
        let msg = msg.to_string();
        let responses = (0..100).map(|_| Err(msg.clone())).collect();
        Self::new(responses)
    }
}

#[async_trait]
impl RssFetcher for MockRssFetcher {
    async fn fetch_and_parse(&self, _feed_url: &str) -> Result<PodcastFeed, AppError> {
        let mut responses = self.responses.lock().unwrap();
        if responses.is_empty() {
            return Err(AppError::Other(
                "MockRssFetcher: no more responses".into(),
            ));
        }
        responses
            .remove(0)
            .map_err(|msg| AppError::RssParse(msg))
    }
}

// ---------------------------------------------------------------------------
// MockFeedUrlResolver
// ---------------------------------------------------------------------------

pub struct MockFeedUrlResolver {
    pub result: Result<String, String>,
}

#[async_trait]
impl FeedUrlResolver for MockFeedUrlResolver {
    async fn resolve_feed_url(&self, _url: &str) -> Result<String, AppError> {
        match &self.result {
            Ok(url) => Ok(url.clone()),
            Err(msg) => Err(AppError::InvalidUrl(msg.clone())),
        }
    }
}

// ---------------------------------------------------------------------------
// MockFileDownloader
// ---------------------------------------------------------------------------

pub struct MockFileDownloader {
    results: Mutex<Vec<Result<(), String>>>,
    pub call_count: Mutex<usize>,
}

impl MockFileDownloader {
    pub fn always_ok() -> Self {
        let results = (0..100).map(|_| Ok(())).collect();
        Self {
            results: Mutex::new(results),
            call_count: Mutex::new(0),
        }
    }

    pub fn always_err(msg: &str) -> Self {
        let msg = msg.to_string();
        let results = (0..100).map(|_| Err(msg.clone())).collect();
        Self {
            results: Mutex::new(results),
            call_count: Mutex::new(0),
        }
    }

    pub fn with_results(results: Vec<Result<(), String>>) -> Self {
        Self {
            results: Mutex::new(results),
            call_count: Mutex::new(0),
        }
    }

    pub fn get_call_count(&self) -> usize {
        *self.call_count.lock().unwrap()
    }
}

#[async_trait]
impl FileDownloader for MockFileDownloader {
    async fn download(
        &self,
        _audio_url: &str,
        _save_path: &Path,
        _episode_id: i64,
        _on_progress: Box<dyn FnMut(DownloadProgress) + Send>,
    ) -> Result<(), AppError> {
        *self.call_count.lock().unwrap() += 1;
        let mut results = self.results.lock().unwrap();
        if results.is_empty() {
            return Err(AppError::Other(
                "MockFileDownloader: no more results".into(),
            ));
        }
        results
            .remove(0)
            .map_err(|msg| AppError::Other(msg))
    }
}

// ---------------------------------------------------------------------------
// MockSettingsStore
// ---------------------------------------------------------------------------

pub struct MockSettingsStore {
    settings: Mutex<AppSettings>,
}

impl MockSettingsStore {
    pub fn with_download_dir(dir: &str) -> Self {
        Self {
            settings: Mutex::new(AppSettings {
                download_dir: Some(dir.to_string()),
                ..AppSettings::default()
            }),
        }
    }

    pub fn without_download_dir() -> Self {
        Self {
            settings: Mutex::new(AppSettings::default()),
        }
    }
}

impl SettingsStore for MockSettingsStore {
    fn load_settings(&self) -> Result<AppSettings, AppError> {
        Ok(self.settings.lock().unwrap().clone())
    }

    fn save_settings(&self, settings: &AppSettings) -> Result<(), AppError> {
        *self.settings.lock().unwrap() = settings.clone();
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// テストデータヘルパー
// ---------------------------------------------------------------------------

pub fn create_test_db_state() -> DbState {
    let conn = db::init_test_db().unwrap();
    DbState(Mutex::new(conn))
}

pub fn setup_podcast_in_state(state: &DbState, title: &str, feed_url: &str) -> i64 {
    let conn = state.0.lock().unwrap();
    let p = db::podcast::insert(&conn, title, None, None, feed_url, None, None).unwrap();
    p.id
}

pub fn setup_episodes_in_state(state: &DbState, podcast_id: i64, episodes: &[NewEpisode]) {
    let conn = state.0.lock().unwrap();
    db::episode::insert_bulk(&conn, podcast_id, episodes).unwrap();
}

pub fn make_episodes(count: usize) -> Vec<NewEpisode> {
    (0..count)
        .map(|i| NewEpisode {
            guid: format!("guid-{i}"),
            title: format!("Episode {i}"),
            description: None,
            audio_url: format!("https://example.com/ep{i}.mp3"),
            duration: None,
            file_size: None,
            published_at: format!("2026-01-{:02}T00:00:00Z", i + 1),
        })
        .collect()
}

pub fn make_feed(title: &str, episode_count: usize) -> PodcastFeed {
    PodcastFeed {
        title: title.to_string(),
        author: None,
        description: None,
        image_url: None,
        episodes: make_episodes(episode_count),
    }
}
