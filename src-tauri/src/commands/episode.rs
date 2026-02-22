use tauri::State;

use crate::db::{self, DbState};
use crate::error::AppError;
use crate::models::episode::Episode;
use crate::models::podcast::PodcastNewCount;
use crate::services::rss;

/// 番組のエピソード一覧を取得する
#[tauri::command]
pub async fn list_episodes(
    podcast_id: i64,
    state: State<'_, DbState>,
) -> Result<Vec<Episode>, AppError> {
    let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
    db::episode::list_by_podcast(&conn, podcast_id)
}

/// 番組の新着エピソードをチェックする
///
/// RSS を再取得し、新規エピソードを DB に挿入してから新着一覧を返す
#[tauri::command]
pub async fn check_new_episodes(
    podcast_id: i64,
    state: State<'_, DbState>,
) -> Result<Vec<Episode>, AppError> {
    // 1. DB から番組情報を取得
    let feed_url = {
        let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
        let podcast = db::podcast::get(&conn, podcast_id)?;
        podcast.feed_url
    };

    // 2. RSS フィードを再取得・パース（ロック外で非同期処理）
    let feed = rss::fetch_and_parse(&feed_url).await?;

    // 3. 新規エピソードを挿入し、新着一覧を取得
    let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
    db::episode::insert_bulk(&conn, podcast_id, &feed.episodes)?;
    let new_episodes = db::episode::get_new_episodes(&conn, podcast_id)?;
    db::podcast::update_last_checked(&conn, podcast_id)?;

    Ok(new_episodes)
}

/// 全番組の新着をチェックする
#[tauri::command]
pub async fn check_all_new(
    state: State<'_, DbState>,
) -> Result<Vec<PodcastNewCount>, AppError> {
    // 1. 全番組の id, title, feed_url を一括取得
    let podcasts: Vec<(i64, String, String)> = {
        let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
        let summaries = db::podcast::list(&conn)?;
        summaries
            .iter()
            .filter_map(|s| {
                db::podcast::get(&conn, s.id)
                    .ok()
                    .map(|p| (p.id, p.title, p.feed_url))
            })
            .collect()
    };

    let mut results = Vec::new();

    for (podcast_id, title, feed_url) in &podcasts {

        // 3. RSS フィードを再取得（ロック外で非同期処理）
        let feed = match rss::fetch_and_parse(&feed_url).await {
            Ok(feed) => feed,
            Err(e) => {
                log::warn!("番組「{}」のフィード取得に失敗、スキップ: {}", title, e);
                continue;
            }
        };

        // 4. 新規エピソードを挿入し、新着数をカウント
        let new_count = {
            let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
            db::episode::insert_bulk(&conn, *podcast_id, &feed.episodes)?;
            let new_episodes = db::episode::get_new_episodes(&conn, *podcast_id)?;
            db::podcast::update_last_checked(&conn, *podcast_id)?;
            new_episodes.len()
        };

        results.push(PodcastNewCount {
            podcast_id: *podcast_id,
            title: title.clone(),
            new_count,
        });
    }

    Ok(results)
}
