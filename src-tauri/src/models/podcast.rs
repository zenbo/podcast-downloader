use serde::{Deserialize, Serialize};

/// データベースから取得した完全な番組情報
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Podcast {
    pub id: i64,
    pub title: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub feed_url: String,
    pub apple_podcasts_url: Option<String>,
    pub image_url: Option<String>,
    pub last_checked_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// 番組一覧表示用（新着エピソード数を含む）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodcastSummary {
    pub id: i64,
    pub title: String,
    pub author: Option<String>,
    pub image_url: Option<String>,
    pub new_episode_count: usize,
}

/// RSS フィードからパースした番組情報（DB 挿入前の中間形式）
#[derive(Debug, Clone)]
pub struct PodcastFeed {
    pub title: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub episodes: Vec<crate::models::episode::NewEpisode>,
}

/// 全番組の新着チェック結果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodcastNewCount {
    pub podcast_id: i64,
    pub title: String,
    pub new_count: usize,
}
