use feed_rs::parser;

use crate::error::AppError;
use crate::models::episode::NewEpisode;
use crate::models::podcast::PodcastFeed;

/// バイト列から PodcastFeed をパースする
pub fn parse_feed(data: &[u8]) -> Result<PodcastFeed, AppError> {
    let feed = parser::parse(data)
        .map_err(|e| AppError::RssParse(e.to_string()))?;

    let title = feed
        .title
        .map(|t| t.content)
        .unwrap_or_default();

    let author = feed.authors.first().map(|a| a.name.clone());

    let description = feed.description.map(|d| d.content);

    let image_url = feed
        .logo
        .map(|l| l.uri)
        .or_else(|| feed.icon.map(|i| i.uri));

    let episodes: Vec<NewEpisode> = feed
        .entries
        .into_iter()
        .filter_map(|entry| {
            // enclosure（音声ファイル）の URL を取得
            let audio_url = entry
                .media
                .first()
                .and_then(|m| m.content.first())
                .and_then(|c| c.url.as_ref())
                .map(|u| u.to_string())
                .or_else(|| {
                    entry
                        .links
                        .iter()
                        .find(|l| {
                            l.media_type
                                .as_deref()
                                .is_some_and(|mt| mt.starts_with("audio/"))
                        })
                        .map(|l| l.href.clone())
                })?;

            let published_at = entry
                .published
                .or(entry.updated)
                .map(|dt| dt.to_rfc3339())?;

            let file_size = entry
                .media
                .first()
                .and_then(|m| m.content.first())
                .and_then(|c| c.size)
                .map(|s| s as i64);

            let duration = entry
                .media
                .first()
                .and_then(|m| m.duration)
                .map(|d| {
                    let total_secs = d.as_secs();
                    let hours = total_secs / 3600;
                    let minutes = (total_secs % 3600) / 60;
                    let seconds = total_secs % 60;
                    if hours > 0 {
                        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
                    } else {
                        format!("{:02}:{:02}", minutes, seconds)
                    }
                });

            Some(NewEpisode {
                guid: entry.id,
                title: entry.title.map(|t| t.content).unwrap_or_default(),
                description: entry.summary.map(|s| s.content),
                audio_url,
                duration,
                file_size,
                published_at,
            })
        })
        .collect();

    Ok(PodcastFeed {
        title,
        author,
        description,
        image_url,
        episodes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 最小限の RSS 2.0 XML を生成する
    fn minimal_rss(channel_inner: &str) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <rss version="2.0" xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd"
                 xmlns:media="http://search.yahoo.com/mrss/">
              <channel>
                {channel_inner}
              </channel>
            </rss>"#
        )
    }

    /// enclosure 付きの item XML を生成する
    fn rss_item(guid: &str, title: &str, audio_url: &str, pub_date: &str) -> String {
        format!(
            r#"<item>
              <guid>{guid}</guid>
              <title>{title}</title>
              <enclosure url="{audio_url}" type="audio/mpeg" length="12345678"/>
              <pubDate>{pub_date}</pubDate>
            </item>"#
        )
    }

    // ─── フィードレベル ───

    #[test]
    fn test_parse_full_feed() {
        let xml = minimal_rss(&format!(
            r#"<title>My Podcast</title>
            <itunes:author>John Doe</itunes:author>
            <description>A great podcast</description>
            <image><url>https://example.com/logo.png</url></image>
            {}
            {}"#,
            rss_item("ep1", "Episode 1", "https://example.com/ep1.mp3", "Mon, 01 Jan 2024 00:00:00 GMT"),
            rss_item("ep2", "Episode 2", "https://example.com/ep2.mp3", "Tue, 02 Jan 2024 00:00:00 GMT"),
        ));

        let result = parse_feed(xml.as_bytes()).unwrap();
        assert_eq!(result.title, "My Podcast");
        assert_eq!(result.episodes.len(), 2);
        assert!(result.description.is_some());
    }

    #[test]
    fn test_feed_optional_fields_absent() {
        let xml = minimal_rss(&rss_item(
            "ep1", "Episode 1", "https://example.com/ep1.mp3", "Mon, 01 Jan 2024 00:00:00 GMT",
        ));

        let result = parse_feed(xml.as_bytes()).unwrap();
        assert_eq!(result.title, "");
        assert!(result.author.is_none());
        assert!(result.description.is_none());
        assert_eq!(result.episodes.len(), 1);
    }

    #[test]
    fn test_feed_image_fallback_to_icon() {
        // RSS の <image><url> は feed-rs で icon にマッピングされる（logo ではない）
        let xml = minimal_rss(&format!(
            r#"<image><url>https://example.com/icon.png</url><title>icon</title><link>https://example.com</link></image>
            {}"#,
            rss_item("ep1", "Episode 1", "https://example.com/ep1.mp3", "Mon, 01 Jan 2024 00:00:00 GMT"),
        ));

        let result = parse_feed(xml.as_bytes()).unwrap();
        // logo がない場合は icon にフォールバックする
        assert!(result.image_url.is_some());
    }

    // ─── エピソード audio_url ───

    #[test]
    fn test_episode_audio_from_enclosure() {
        let xml = minimal_rss(&rss_item(
            "ep1", "Episode 1", "https://example.com/ep1.mp3", "Mon, 01 Jan 2024 00:00:00 GMT",
        ));

        let result = parse_feed(xml.as_bytes()).unwrap();
        assert_eq!(result.episodes.len(), 1);
        assert_eq!(result.episodes[0].audio_url, "https://example.com/ep1.mp3");
        assert_eq!(result.episodes[0].guid, "ep1");
        assert_eq!(result.episodes[0].title, "Episode 1");
        assert_eq!(result.episodes[0].file_size, Some(12345678));
    }

    #[test]
    fn test_episode_skipped_without_audio() {
        let xml = minimal_rss(
            r#"<item>
              <guid>ep1</guid>
              <title>No Audio</title>
              <pubDate>Mon, 01 Jan 2024 00:00:00 GMT</pubDate>
            </item>"#,
        );

        let result = parse_feed(xml.as_bytes()).unwrap();
        assert_eq!(result.episodes.len(), 0);
    }

    // ─── エピソード日付 ───

    #[test]
    fn test_episode_published_at() {
        let xml = minimal_rss(&rss_item(
            "ep1", "Episode 1", "https://example.com/ep1.mp3", "Mon, 01 Jan 2024 12:30:00 +0900",
        ));

        let result = parse_feed(xml.as_bytes()).unwrap();
        let pub_at = &result.episodes[0].published_at;
        // RFC3339 形式であることを確認
        assert!(pub_at.contains("2024-01-01"), "published_at should contain date: {pub_at}");
    }

    #[test]
    fn test_episode_skipped_without_date() {
        let xml = minimal_rss(
            r#"<item>
              <guid>ep1</guid>
              <title>No Date</title>
              <enclosure url="https://example.com/ep1.mp3" type="audio/mpeg" length="100"/>
            </item>"#,
        );

        let result = parse_feed(xml.as_bytes()).unwrap();
        assert_eq!(result.episodes.len(), 0);
    }

    // ─── Duration ───

    #[test]
    fn test_duration_format() {
        // itunes:duration は feed-rs で media[0].duration にマッピングされる
        let xml = minimal_rss(
            r#"<item>
              <guid>ep-long</guid>
              <title>Long Episode</title>
              <enclosure url="https://example.com/long.mp3" type="audio/mpeg" length="100"/>
              <pubDate>Mon, 01 Jan 2024 00:00:00 GMT</pubDate>
              <itunes:duration>01:01:01</itunes:duration>
            </item>
            <item>
              <guid>ep-short</guid>
              <title>Short Episode</title>
              <enclosure url="https://example.com/short.mp3" type="audio/mpeg" length="100"/>
              <pubDate>Tue, 02 Jan 2024 00:00:00 GMT</pubDate>
              <itunes:duration>125</itunes:duration>
            </item>
            <item>
              <guid>ep-no-dur</guid>
              <title>No Duration</title>
              <enclosure url="https://example.com/nodur.mp3" type="audio/mpeg" length="100"/>
              <pubDate>Wed, 03 Jan 2024 00:00:00 GMT</pubDate>
            </item>"#,
        );

        let result = parse_feed(xml.as_bytes()).unwrap();
        assert_eq!(result.episodes.len(), 3);

        let long = result.episodes.iter().find(|e| e.guid == "ep-long").unwrap();
        let short = result.episodes.iter().find(|e| e.guid == "ep-short").unwrap();
        let no_dur = result.episodes.iter().find(|e| e.guid == "ep-no-dur").unwrap();

        // HH:MM:SS → 1時間超なので HH:MM:SS 形式
        assert_eq!(long.duration.as_deref(), Some("01:01:01"));
        // 125秒 → 1時間未満なので MM:SS 形式
        assert_eq!(short.duration.as_deref(), Some("02:05"));
        // duration なし → None
        assert!(no_dur.duration.is_none());
    }

    // ─── フィルタリング複合 ───

    #[test]
    fn test_mixed_valid_and_invalid_entries() {
        let xml = minimal_rss(&format!(
            r#"{}
            {}
            <item>
              <guid>no-audio</guid>
              <title>No Audio</title>
              <pubDate>Wed, 03 Jan 2024 00:00:00 GMT</pubDate>
            </item>
            <item>
              <guid>no-date</guid>
              <title>No Date</title>
              <enclosure url="https://example.com/nodate.mp3" type="audio/mpeg" length="100"/>
            </item>"#,
            rss_item("ep1", "Valid 1", "https://example.com/ep1.mp3", "Mon, 01 Jan 2024 00:00:00 GMT"),
            rss_item("ep2", "Valid 2", "https://example.com/ep2.mp3", "Tue, 02 Jan 2024 00:00:00 GMT"),
        ));

        let result = parse_feed(xml.as_bytes()).unwrap();
        assert_eq!(result.episodes.len(), 2);
        assert!(result.episodes.iter().all(|e| e.guid == "ep1" || e.guid == "ep2"));
    }

    #[test]
    fn test_empty_feed() {
        let xml = minimal_rss("<title>Empty Podcast</title>");

        let result = parse_feed(xml.as_bytes()).unwrap();
        assert_eq!(result.title, "Empty Podcast");
        assert_eq!(result.episodes.len(), 0);
    }

    // ─── エラー系 ───

    #[test]
    fn test_parse_invalid_xml() {
        let result = parse_feed(b"this is not xml");
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::RssParse(_) => {}
            other => panic!("Expected AppError::RssParse, got: {other:?}"),
        }
    }
}

/// RSS フィードを取得・パースし、PodcastFeed として返す
pub async fn fetch_and_parse(feed_url: &str) -> Result<PodcastFeed, AppError> {
    let body = reqwest::get(feed_url)
        .await?
        .bytes()
        .await?;
    parse_feed(&body)
}
