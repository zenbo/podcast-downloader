use serde::{Deserialize, Serialize};

/// データベースから取得した完全なエピソード情報
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    pub id: i64,
    pub podcast_id: i64,
    pub guid: String,
    pub title: String,
    pub description: Option<String>,
    pub audio_url: String,
    pub duration: Option<String>,
    pub file_size: Option<i64>,
    pub published_at: String,
    pub downloaded_at: Option<String>,
    pub created_at: String,
}

/// RSS からパースしたエピソード（DB 挿入用の中間形式）
#[derive(Debug, Clone)]
pub struct NewEpisode {
    pub guid: String,
    pub title: String,
    pub description: Option<String>,
    pub audio_url: String,
    pub duration: Option<String>,
    pub file_size: Option<i64>,
    pub published_at: String,
}

/// 個別ダウンロード進捗通知用
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadProgress {
    pub episode_id: i64,
    pub downloaded_bytes: u64,
    pub total_bytes: Option<u64>,
    pub percentage: Option<f64>,
}

/// 一括ダウンロード進捗通知用
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchDownloadProgress {
    pub current_episode_id: i64,
    pub current_episode_title: String,
    pub episode_progress: DownloadProgress,
    pub completed_count: usize,
    pub total_count: usize,
}
