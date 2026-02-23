use rusqlite::Connection;
use tauri::State;

use crate::db::{self, DbState};
use crate::error::AppError;
use crate::models::podcast::{Podcast, PodcastSummary};
use crate::services::traits::ServiceContainer;

/// Apple Podcasts URL から番組を登録する
#[tauri::command]
pub async fn register_podcast(
    url: String,
    state: State<'_, DbState>,
    services: State<'_, ServiceContainer>,
) -> Result<Podcast, AppError> {
    // 1. 外部 IO（ロック外で非同期処理）
    let feed_url = services.feed_url_resolver.resolve_feed_url(&url).await?;
    let feed = services.rss_fetcher.fetch_and_parse(&feed_url).await?;

    // 2. DB 操作
    let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
    register_podcast_impl(&conn, &url, &feed_url, feed)
}

/// register_podcast のテスト可能なロジック部分
pub(crate) fn register_podcast_impl(
    conn: &Connection,
    apple_podcasts_url: &str,
    feed_url: &str,
    feed: crate::models::podcast::PodcastFeed,
) -> Result<Podcast, AppError> {
    let podcast = db::podcast::insert(
        conn,
        &feed.title,
        feed.author.as_deref(),
        feed.description.as_deref(),
        feed_url,
        Some(apple_podcasts_url),
        feed.image_url.as_deref(),
    )?;
    db::episode::insert_bulk(conn, podcast.id, &feed.episodes)?;
    Ok(podcast)
}

/// 番組一覧を取得する（新着エピソード数付き）
#[tauri::command]
pub async fn list_podcasts(
    state: State<'_, DbState>,
) -> Result<Vec<PodcastSummary>, AppError> {
    let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
    db::podcast::list(&conn)
}

/// 番組を削除する
#[tauri::command]
pub async fn delete_podcast(
    podcast_id: i64,
    state: State<'_, DbState>,
) -> Result<(), AppError> {
    let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
    db::podcast::delete(&conn, podcast_id)
}
