use rusqlite::Connection;
use tauri::State;

use crate::db::{self, DbState};
use crate::error::AppError;
use crate::models::podcast::{Podcast, PodcastSummary};
use crate::services::traits::{FeedUrlResolver, RssFetcher, ServiceContainer};

/// Apple Podcasts URL から番組を登録する
#[tauri::command]
pub async fn register_podcast(
    url: String,
    state: State<'_, DbState>,
    services: State<'_, ServiceContainer>,
) -> Result<Podcast, AppError> {
    register_podcast_workflow(
        &state,
        &*services.feed_url_resolver,
        &*services.rss_fetcher,
        &url,
    )
    .await
}

/// register_podcast のテスト可能なワークフロー全体
pub(crate) async fn register_podcast_workflow(
    state: &DbState,
    feed_url_resolver: &dyn FeedUrlResolver,
    rss_fetcher: &dyn RssFetcher,
    url: &str,
) -> Result<Podcast, AppError> {
    let feed_url = feed_url_resolver.resolve_feed_url(url).await?;
    let feed = rss_fetcher.fetch_and_parse(&feed_url).await?;
    let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
    register_podcast_impl(&conn, url, &feed_url, feed)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::test_helpers::*;

    #[test]
    fn test_register_podcast_impl_success() {
        let state = create_test_db_state();
        let feed = make_feed("Test Podcast", 3);

        let conn = state.0.lock().unwrap();
        let podcast = register_podcast_impl(
            &conn,
            "https://podcasts.apple.com/podcast/id12345",
            "https://example.com/feed.xml",
            feed,
        )
        .unwrap();

        assert_eq!(podcast.title, "Test Podcast");
        assert_eq!(podcast.feed_url, "https://example.com/feed.xml");
        assert_eq!(
            podcast.apple_podcasts_url.as_deref(),
            Some("https://podcasts.apple.com/podcast/id12345")
        );

        let episodes = db::episode::list_by_podcast(&conn, podcast.id).unwrap();
        assert_eq!(episodes.len(), 3);
    }

    #[tokio::test]
    async fn test_register_podcast_rss_error_prevents_db_insert() {
        let state = create_test_db_state();
        let resolver = MockFeedUrlResolver {
            result: Ok("https://example.com/feed.xml".to_string()),
        };
        let fetcher = MockRssFetcher::always_err("parse error");

        let result = register_podcast_workflow(
            &state,
            &resolver,
            &fetcher,
            "https://podcasts.apple.com/podcast/id12345",
        )
        .await;

        assert!(result.is_err());

        // DB に番組が登録されていないことを確認
        let conn = state.0.lock().unwrap();
        let podcasts = db::podcast::list(&conn).unwrap();
        assert!(podcasts.is_empty());
    }

    #[tokio::test]
    async fn test_register_podcast_feed_url_resolution_error() {
        let state = create_test_db_state();
        let resolver = MockFeedUrlResolver {
            result: Err("invalid url".to_string()),
        };
        let fetcher = MockRssFetcher::always_ok(make_feed("X", 0));

        let result =
            register_podcast_workflow(&state, &resolver, &fetcher, "not-a-valid-url").await;

        assert!(result.is_err());

        // DB に番組が登録されていないことを確認
        let conn = state.0.lock().unwrap();
        let podcasts = db::podcast::list(&conn).unwrap();
        assert!(podcasts.is_empty());
    }
}
