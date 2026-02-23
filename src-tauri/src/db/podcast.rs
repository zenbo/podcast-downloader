use rusqlite::{params, Connection, Row};

use crate::error::AppError;
use crate::models::podcast::{Podcast, PodcastSummary};

fn row_to_podcast(row: &Row) -> rusqlite::Result<Podcast> {
    Ok(Podcast {
        id: row.get("id")?,
        title: row.get("title")?,
        author: row.get("author")?,
        description: row.get("description")?,
        feed_url: row.get("feed_url")?,
        apple_podcasts_url: row.get("apple_podcasts_url")?,
        image_url: row.get("image_url")?,
        last_checked_at: row.get("last_checked_at")?,
        created_at: row.get("created_at")?,
        updated_at: row.get("updated_at")?,
    })
}

/// 番組を登録し、挿入された行を返す
pub fn insert(
    conn: &Connection,
    title: &str,
    author: Option<&str>,
    description: Option<&str>,
    feed_url: &str,
    apple_podcasts_url: Option<&str>,
    image_url: Option<&str>,
) -> Result<Podcast, AppError> {
    conn.execute(
        "INSERT INTO podcasts (title, author, description, feed_url, apple_podcasts_url, image_url)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            title,
            author,
            description,
            feed_url,
            apple_podcasts_url,
            image_url
        ],
    )?;
    let id = conn.last_insert_rowid();
    get(conn, id)
}

/// 番組一覧を新着エピソード数付きで取得する
pub fn list(conn: &Connection) -> Result<Vec<PodcastSummary>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT p.id, p.title, p.author, p.image_url,
           (SELECT COUNT(*) FROM episodes e
            WHERE e.podcast_id = p.id
              AND e.downloaded_at IS NULL
              AND e.published_at >= COALESCE(
                (SELECT e2.published_at FROM episodes e2
                 WHERE e2.podcast_id = p.id AND e2.downloaded_at IS NOT NULL
                 ORDER BY e2.published_at DESC LIMIT 1),
                '1970-01-01'
              )
           ) AS new_episode_count
         FROM podcasts p
         ORDER BY p.created_at DESC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(PodcastSummary {
            id: row.get("id")?,
            title: row.get("title")?,
            author: row.get("author")?,
            image_url: row.get("image_url")?,
            new_episode_count: row.get::<_, i64>("new_episode_count")? as usize,
        })
    })?;
    let mut podcasts = Vec::new();
    for row in rows {
        podcasts.push(row?);
    }
    Ok(podcasts)
}

/// 全番組の完全な情報を取得する
pub fn list_all(conn: &Connection) -> Result<Vec<Podcast>, AppError> {
    let mut stmt = conn.prepare("SELECT * FROM podcasts ORDER BY created_at DESC")?;
    let rows = stmt.query_map([], row_to_podcast)?;
    let mut podcasts = Vec::new();
    for row in rows {
        podcasts.push(row?);
    }
    Ok(podcasts)
}

/// 番組を ID で取得する
pub fn get(conn: &Connection, id: i64) -> Result<Podcast, AppError> {
    let mut stmt = conn.prepare("SELECT * FROM podcasts WHERE id = ?1")?;
    let podcast = stmt.query_row(params![id], row_to_podcast)?;
    Ok(podcast)
}

/// 番組を削除する（CASCADE でエピソードも削除される）
pub fn delete(conn: &Connection, id: i64) -> Result<(), AppError> {
    conn.execute("DELETE FROM podcasts WHERE id = ?1", params![id])?;
    Ok(())
}

/// 最終新着チェック日時を更新する
pub fn update_last_checked(conn: &Connection, id: i64) -> Result<(), AppError> {
    conn.execute(
        "UPDATE podcasts SET last_checked_at = datetime('now'), updated_at = datetime('now') WHERE id = ?1",
        params![id],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_test_db;

    #[test]
    fn test_insert_and_get() {
        let conn = init_test_db().unwrap();
        let podcast = insert(
            &conn,
            "Test Podcast",
            Some("Author"),
            Some("Description"),
            "https://example.com/feed.xml",
            Some("https://podcasts.apple.com/podcast/id12345"),
            Some("https://example.com/image.jpg"),
        )
        .unwrap();

        assert_eq!(podcast.title, "Test Podcast");
        assert_eq!(podcast.author.as_deref(), Some("Author"));
        assert_eq!(podcast.feed_url, "https://example.com/feed.xml");

        let fetched = get(&conn, podcast.id).unwrap();
        assert_eq!(fetched.title, podcast.title);
        assert_eq!(fetched.feed_url, podcast.feed_url);
    }

    #[test]
    fn test_list_empty() {
        let conn = init_test_db().unwrap();
        let podcasts = list(&conn).unwrap();
        assert!(podcasts.is_empty());
    }

    #[test]
    fn test_list_with_podcasts() {
        let conn = init_test_db().unwrap();
        insert(
            &conn,
            "Podcast A",
            None,
            None,
            "https://a.com/feed",
            None,
            None,
        )
        .unwrap();
        insert(
            &conn,
            "Podcast B",
            None,
            None,
            "https://b.com/feed",
            None,
            None,
        )
        .unwrap();

        let podcasts = list(&conn).unwrap();
        assert_eq!(podcasts.len(), 2);
        let titles: Vec<&str> = podcasts.iter().map(|p| p.title.as_str()).collect();
        assert!(titles.contains(&"Podcast A"));
        assert!(titles.contains(&"Podcast B"));
        assert_eq!(podcasts[0].new_episode_count, 0);
    }

    #[test]
    fn test_delete() {
        let conn = init_test_db().unwrap();
        let podcast = insert(
            &conn,
            "To Delete",
            None,
            None,
            "https://del.com/feed",
            None,
            None,
        )
        .unwrap();
        delete(&conn, podcast.id).unwrap();

        let result = get(&conn, podcast.id);
        assert!(result.is_err());
    }

    #[test]
    fn test_list_all_empty() {
        let conn = init_test_db().unwrap();
        let podcasts = list_all(&conn).unwrap();
        assert!(podcasts.is_empty());
    }

    #[test]
    fn test_list_all_returns_full_records() {
        let conn = init_test_db().unwrap();
        insert(
            &conn,
            "Podcast A",
            Some("Author A"),
            None,
            "https://a.com/feed",
            None,
            None,
        )
        .unwrap();
        insert(
            &conn,
            "Podcast B",
            None,
            Some("Desc B"),
            "https://b.com/feed",
            None,
            None,
        )
        .unwrap();

        let podcasts = list_all(&conn).unwrap();
        assert_eq!(podcasts.len(), 2);

        // list_all は Podcast 型（feed_url を含む完全な情報）を返す
        let feed_urls: Vec<&str> = podcasts.iter().map(|p| p.feed_url.as_str()).collect();
        assert!(feed_urls.contains(&"https://a.com/feed"));
        assert!(feed_urls.contains(&"https://b.com/feed"));
    }

    #[test]
    fn test_update_last_checked() {
        let conn = init_test_db().unwrap();
        let podcast = insert(
            &conn,
            "Check Me",
            None,
            None,
            "https://check.com/feed",
            None,
            None,
        )
        .unwrap();
        assert!(podcast.last_checked_at.is_none());

        update_last_checked(&conn, podcast.id).unwrap();
        let updated = get(&conn, podcast.id).unwrap();
        assert!(updated.last_checked_at.is_some());
    }
}
