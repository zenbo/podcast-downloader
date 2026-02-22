use rusqlite::{params, Connection, Row};

use crate::error::AppError;
use crate::models::episode::{Episode, NewEpisode};

fn row_to_episode(row: &Row) -> rusqlite::Result<Episode> {
    Ok(Episode {
        id: row.get("id")?,
        podcast_id: row.get("podcast_id")?,
        guid: row.get("guid")?,
        title: row.get("title")?,
        description: row.get("description")?,
        audio_url: row.get("audio_url")?,
        duration: row.get("duration")?,
        file_size: row.get("file_size")?,
        published_at: row.get("published_at")?,
        downloaded_at: row.get("downloaded_at")?,
        created_at: row.get("created_at")?,
    })
}

/// 複数エピソードを一括挿入する（重複は無視）
pub fn insert_bulk(
    conn: &Connection,
    podcast_id: i64,
    episodes: &[NewEpisode],
) -> Result<(), AppError> {
    let mut stmt = conn.prepare(
        "INSERT OR IGNORE INTO episodes
         (podcast_id, guid, title, description, audio_url, duration, file_size, published_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
    )?;
    for ep in episodes {
        stmt.execute(params![
            podcast_id,
            ep.guid,
            ep.title,
            ep.description,
            ep.audio_url,
            ep.duration,
            ep.file_size,
            ep.published_at,
        ])?;
    }
    Ok(())
}

/// 番組のエピソード一覧を配信日降順で取得する
pub fn list_by_podcast(
    conn: &Connection,
    podcast_id: i64,
) -> Result<Vec<Episode>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT * FROM episodes WHERE podcast_id = ?1 ORDER BY published_at DESC",
    )?;
    let rows = stmt.query_map(params![podcast_id], row_to_episode)?;
    let mut episodes = Vec::new();
    for row in rows {
        episodes.push(row?);
    }
    Ok(episodes)
}

/// エピソードを ID で取得する
pub fn get(conn: &Connection, id: i64) -> Result<Episode, AppError> {
    let mut stmt = conn.prepare("SELECT * FROM episodes WHERE id = ?1")?;
    let episode = stmt.query_row(params![id], row_to_episode)?;
    Ok(episode)
}

/// エピソードのダウンロード完了日時を記録する
pub fn mark_downloaded(conn: &Connection, id: i64) -> Result<(), AppError> {
    conn.execute(
        "UPDATE episodes SET downloaded_at = datetime('now') WHERE id = ?1",
        params![id],
    )?;
    Ok(())
}

/// 新着エピソードを取得する（03-data-design セクション 5 の判定ロジック）
///
/// - DL 履歴あり: 最後に DL したエピソードの配信日以降かつ未 DL のエピソード
/// - DL 履歴なし: 全未 DL エピソード
pub fn get_new_episodes(
    conn: &Connection,
    podcast_id: i64,
) -> Result<Vec<Episode>, AppError> {
    let has_downloaded: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM episodes WHERE podcast_id = ?1 AND downloaded_at IS NOT NULL)",
        params![podcast_id],
        |row| row.get(0),
    )?;

    let sql = if has_downloaded {
        "SELECT e.*
         FROM episodes e
         WHERE e.podcast_id = ?1
           AND e.published_at >= (
             SELECT e2.published_at
             FROM episodes e2
             WHERE e2.podcast_id = ?1
               AND e2.downloaded_at IS NOT NULL
             ORDER BY e2.published_at DESC
             LIMIT 1
           )
           AND e.downloaded_at IS NULL
         ORDER BY e.published_at ASC"
    } else {
        "SELECT e.*
         FROM episodes e
         WHERE e.podcast_id = ?1
           AND e.downloaded_at IS NULL
         ORDER BY e.published_at ASC"
    };

    let mut stmt = conn.prepare(sql)?;
    let rows = stmt.query_map(params![podcast_id], row_to_episode)?;
    let mut episodes = Vec::new();
    for row in rows {
        episodes.push(row?);
    }
    Ok(episodes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{init_test_db, podcast};

    fn setup_podcast(conn: &Connection) -> i64 {
        let p = podcast::insert(
            conn,
            "Test Podcast",
            None,
            None,
            "https://example.com/feed.xml",
            None,
            None,
        )
        .unwrap();
        p.id
    }

    fn make_episodes(count: usize) -> Vec<NewEpisode> {
        (0..count)
            .map(|i| NewEpisode {
                guid: format!("guid-{i}"),
                title: format!("Episode {i}"),
                description: None,
                audio_url: format!("https://example.com/ep{i}.mp3"),
                duration: None,
                file_size: None,
                published_at: format!("2026-01-{:02}T00:00:00Z", i + 1),
            })
            .collect()
    }

    #[test]
    fn test_insert_bulk_and_list() {
        let conn = init_test_db().unwrap();
        let pid = setup_podcast(&conn);
        let episodes = make_episodes(3);

        insert_bulk(&conn, pid, &episodes).unwrap();
        let listed = list_by_podcast(&conn, pid).unwrap();
        assert_eq!(listed.len(), 3);
        // published_at DESC なので最新が先
        assert_eq!(listed[0].title, "Episode 2");
    }

    #[test]
    fn test_insert_bulk_ignores_duplicates() {
        let conn = init_test_db().unwrap();
        let pid = setup_podcast(&conn);
        let episodes = make_episodes(2);

        insert_bulk(&conn, pid, &episodes).unwrap();
        insert_bulk(&conn, pid, &episodes).unwrap(); // 重複挿入
        let listed = list_by_podcast(&conn, pid).unwrap();
        assert_eq!(listed.len(), 2);
    }

    #[test]
    fn test_get_episode() {
        let conn = init_test_db().unwrap();
        let pid = setup_podcast(&conn);
        insert_bulk(&conn, pid, &make_episodes(1)).unwrap();

        let listed = list_by_podcast(&conn, pid).unwrap();
        let ep = get(&conn, listed[0].id).unwrap();
        assert_eq!(ep.guid, "guid-0");
    }

    #[test]
    fn test_mark_downloaded() {
        let conn = init_test_db().unwrap();
        let pid = setup_podcast(&conn);
        insert_bulk(&conn, pid, &make_episodes(1)).unwrap();

        let listed = list_by_podcast(&conn, pid).unwrap();
        assert!(listed[0].downloaded_at.is_none());

        mark_downloaded(&conn, listed[0].id).unwrap();
        let ep = get(&conn, listed[0].id).unwrap();
        assert!(ep.downloaded_at.is_some());
    }

    #[test]
    fn test_new_episodes_no_download_history() {
        let conn = init_test_db().unwrap();
        let pid = setup_podcast(&conn);
        insert_bulk(&conn, pid, &make_episodes(3)).unwrap();

        // DL 履歴なし → 全エピソードが新着
        let new_eps = get_new_episodes(&conn, pid).unwrap();
        assert_eq!(new_eps.len(), 3);
    }

    #[test]
    fn test_new_episodes_with_download_history() {
        let conn = init_test_db().unwrap();
        let pid = setup_podcast(&conn);
        insert_bulk(&conn, pid, &make_episodes(5)).unwrap();

        // Episode 2 (published 2026-01-03) を DL 済みにする
        let listed = list_by_podcast(&conn, pid).unwrap();
        // listed は DESC なので: [4, 3, 2, 1, 0]
        let ep2 = listed.iter().find(|e| e.guid == "guid-2").unwrap();
        mark_downloaded(&conn, ep2.id).unwrap();

        // 新着: published_at >= 2026-01-03 かつ未 DL → Episode 3, 4
        let new_eps = get_new_episodes(&conn, pid).unwrap();
        assert_eq!(new_eps.len(), 2);
        assert_eq!(new_eps[0].guid, "guid-3");
        assert_eq!(new_eps[1].guid, "guid-4");
    }

    #[test]
    fn test_cascade_delete() {
        let conn = init_test_db().unwrap();
        let pid = setup_podcast(&conn);
        insert_bulk(&conn, pid, &make_episodes(3)).unwrap();

        podcast::delete(&conn, pid).unwrap();
        let listed = list_by_podcast(&conn, pid).unwrap();
        assert!(listed.is_empty());
    }
}
