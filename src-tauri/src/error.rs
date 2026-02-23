use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("データベースエラー: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("HTTP通信エラー: {0}")]
    Http(#[from] reqwest::Error),

    #[error("RSSパースエラー: {0}")]
    RssParse(String),

    #[cfg(test)]
    #[error("無効なURL: {0}")]
    InvalidUrl(String),

    #[error("Podcast IDを抽出できません: {0}")]
    PodcastIdNotFound(String),

    #[error("RSSフィードURLが見つかりません")]
    FeedUrlNotFound,

    #[error("ファイル操作エラー: {0}")]
    FileSystem(#[from] std::io::Error),

    #[error("{0}")]
    Other(String),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
