use rusqlite::Connection;
use tauri::State;

use crate::db::{self, DbState};
use crate::error::AppError;
use crate::models::episode::Episode;
use crate::models::podcast::PodcastNewCount;
use crate::services::traits::{RssFetcher, ServiceContainer};

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
    services: State<'_, ServiceContainer>,
) -> Result<Vec<Episode>, AppError> {
    // 1. DB から番組情報を取得
    let feed_url = {
        let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
        let podcast = db::podcast::get(&conn, podcast_id)?;
        podcast.feed_url
    };

    // 2. RSS フィードを再取得・パース（ロック外で非同期処理）
    let feed = services.rss_fetcher.fetch_and_parse(&feed_url).await?;

    // 3. 新規エピソードを挿入し、新着一覧を取得
    let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
    check_new_episodes_impl(&conn, podcast_id, feed)
}

/// check_new_episodes のテスト可能なロジック部分（DB 操作）
pub(crate) fn check_new_episodes_impl(
    conn: &Connection,
    podcast_id: i64,
    feed: crate::models::podcast::PodcastFeed,
) -> Result<Vec<Episode>, AppError> {
    db::episode::insert_bulk(conn, podcast_id, &feed.episodes)?;
    let new_episodes = db::episode::get_new_episodes(conn, podcast_id)?;
    db::podcast::update_last_checked(conn, podcast_id)?;
    Ok(new_episodes)
}

/// 全番組の新着をチェックする
#[tauri::command]
pub async fn check_all_new(
    state: State<'_, DbState>,
    services: State<'_, ServiceContainer>,
) -> Result<Vec<PodcastNewCount>, AppError> {
    // 1. 全番組の完全な情報を 1 クエリで取得
    let podcasts = {
        let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
        db::podcast::list_all(&conn)?
    };

    check_all_new_impl(&state, &*services.rss_fetcher, &podcasts).await
}

/// check_all_new のテスト可能なロジック部分
pub(crate) async fn check_all_new_impl(
    state: &DbState,
    rss_fetcher: &dyn RssFetcher,
    podcasts: &[crate::models::podcast::Podcast],
) -> Result<Vec<PodcastNewCount>, AppError> {
    let mut results = Vec::new();

    for podcast in podcasts {
        // RSS フィードを再取得（ロック外で非同期処理）
        let feed = match rss_fetcher.fetch_and_parse(&podcast.feed_url).await {
            Ok(feed) => feed,
            Err(e) => {
                log::warn!(
                    "番組「{}」のフィード取得に失敗、スキップ: {}",
                    podcast.title,
                    e
                );
                continue;
            }
        };

        // 新規エピソードを挿入し、新着数をカウント
        let new_count = {
            let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
            db::episode::insert_bulk(&conn, podcast.id, &feed.episodes)?;
            let new_episodes = db::episode::get_new_episodes(&conn, podcast.id)?;
            db::podcast::update_last_checked(&conn, podcast.id)?;
            new_episodes.len()
        };

        results.push(PodcastNewCount {
            podcast_id: podcast.id,
            title: podcast.title.clone(),
            new_count,
        });
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::test_helpers::*;
    use crate::db;

    #[test]
    fn test_check_new_episodes_impl_inserts_new() {
        let state = create_test_db_state();
        let podcast_id =
            setup_podcast_in_state(&state, "Podcast", "https://example.com/feed");
        let feed = make_feed("Podcast", 3);

        let conn = state.0.lock().unwrap();
        let new_episodes = check_new_episodes_impl(&conn, podcast_id, feed).unwrap();

        // DL 履歴なし → 全エピソードが新着
        assert_eq!(new_episodes.len(), 3);
    }

    #[test]
    fn test_check_new_episodes_impl_ignores_duplicates() {
        let state = create_test_db_state();
        let podcast_id =
            setup_podcast_in_state(&state, "Podcast", "https://example.com/feed");

        let feed1 = make_feed("Podcast", 3);
        let feed2 = make_feed("Podcast", 5); // guid-0 ~ guid-4、うち guid-0~2 は重複

        let conn = state.0.lock().unwrap();
        check_new_episodes_impl(&conn, podcast_id, feed1).unwrap();
        let _new_episodes = check_new_episodes_impl(&conn, podcast_id, feed2).unwrap();

        // 重複は無視され、合計 5 件になる
        let all = db::episode::list_by_podcast(&conn, podcast_id).unwrap();
        assert_eq!(all.len(), 5);
    }

    #[tokio::test]
    async fn test_check_all_new_impl_success() {
        let state = create_test_db_state();
        let _pid1 =
            setup_podcast_in_state(&state, "Podcast A", "https://a.com/feed");
        let _pid2 =
            setup_podcast_in_state(&state, "Podcast B", "https://b.com/feed");

        let podcasts = {
            let conn = state.0.lock().unwrap();
            db::podcast::list_all(&conn).unwrap()
        };

        // list_all は created_at DESC なので podcasts[0] が Podcast B
        let fetcher = MockRssFetcher::new(
            podcasts
                .iter()
                .map(|p| Ok(make_feed(&p.title, 2)))
                .collect(),
        );

        let results = check_all_new_impl(&state, &fetcher, &podcasts)
            .await
            .unwrap();

        assert_eq!(results.len(), 2);
        for r in &results {
            assert_eq!(r.new_count, 2);
        }
    }

    #[tokio::test]
    async fn test_check_all_new_impl_continues_on_rss_failure() {
        let state = create_test_db_state();
        let _pid1 =
            setup_podcast_in_state(&state, "Podcast A", "https://a.com/feed");
        let _pid2 =
            setup_podcast_in_state(&state, "Podcast B", "https://b.com/feed");

        let podcasts = {
            let conn = state.0.lock().unwrap();
            db::podcast::list_all(&conn).unwrap()
        };

        // 1 番組目は失敗、2 番組目は成功
        let fetcher = MockRssFetcher::new(vec![
            Err("network error".to_string()),
            Ok(make_feed(&podcasts[1].title, 2)),
        ]);

        let results = check_all_new_impl(&state, &fetcher, &podcasts)
            .await
            .unwrap();

        // 失敗した番組はスキップされ、成功した番組のみ結果に含まれる
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].new_count, 2);
    }

    #[tokio::test]
    async fn test_check_all_new_impl_all_failures_returns_empty() {
        let state = create_test_db_state();
        let _pid1 =
            setup_podcast_in_state(&state, "Podcast A", "https://a.com/feed");

        let podcasts = {
            let conn = state.0.lock().unwrap();
            db::podcast::list_all(&conn).unwrap()
        };

        let fetcher = MockRssFetcher::always_err("all fail");

        let results = check_all_new_impl(&state, &fetcher, &podcasts)
            .await
            .unwrap();

        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn test_check_all_new_impl_empty_podcasts() {
        let state = create_test_db_state();
        let fetcher = MockRssFetcher::always_ok(make_feed("X", 0));

        let results = check_all_new_impl(&state, &fetcher, &[])
            .await
            .unwrap();

        assert!(results.is_empty());
    }
}
