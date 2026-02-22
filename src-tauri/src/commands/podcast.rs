use tauri::State;

use crate::db::{self, DbState};
use crate::error::AppError;
use crate::models::podcast::{Podcast, PodcastSummary};
use crate::services::{apple_podcasts, rss};

/// Apple Podcasts URL から番組を登録する
#[tauri::command]
pub async fn register_podcast(
    url: String,
    state: State<'_, DbState>,
) -> Result<Podcast, AppError> {
    // 1. Apple Podcasts URL → RSS フィード URL を解決
    let feed_url = apple_podcasts::resolve_feed_url(&url).await?;

    // 2. RSS フィードを取得・パース
    let feed = rss::fetch_and_parse(&feed_url).await?;

    // 3. DB に番組を挿入
    let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
    let podcast = db::podcast::insert(
        &conn,
        &feed.title,
        feed.author.as_deref(),
        feed.description.as_deref(),
        &feed_url,
        Some(url.as_str()),
        feed.image_url.as_deref(),
    )?;

    // 4. エピソードを一括挿入
    db::episode::insert_bulk(&conn, podcast.id, &feed.episodes)?;

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
