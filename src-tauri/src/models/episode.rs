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
    pub file_size: Option<i64>,
    pub published_at: String,
    pub downloaded_at: Option<String>,
    pub created_at: String,
    /// バックエンドの新着判定ロジックに基づく新着フラグ（DB カラムではない）
    pub is_new: bool,
}

/// RSS からパースしたエピソード（DB 挿入用の中間形式）
#[derive(Debug, Clone)]
pub struct NewEpisode {
    pub guid: String,
    pub title: String,
    pub description: Option<String>,
    pub audio_url: String,
    pub file_size: Option<i64>,
    pub published_at: String,
}

/// 新着チェック結果（単一番組）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckNewResult {
    /// 現在の新着エピソード数（既存 + 今回発見）
    pub new_count: usize,
    /// 今回のチェックで新たに見つかったエピソード数
    pub newly_found_count: usize,
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

/// 一括ダウンロード結果サマリー
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchDownloadSummary {
    pub completed_count: usize,
    pub failed_count: usize,
    pub total_count: usize,
}
