use feed_rs::model::Entry;
use feed_rs::parser;

use crate::error::AppError;
use crate::models::episode::NewEpisode;
use crate::models::podcast::PodcastFeed;

const AUDIO_EXTENSIONS: &[&str] = &[".mp3", ".m4a", ".ogg", ".opus", ".wav", ".aac", ".flac"];

/// URL パスが音声ファイル拡張子を持つかを判定する
fn has_audio_extension(url_path: &str) -> bool {
    let path_lower = url_path.to_lowercase();
    AUDIO_EXTENSIONS.iter().any(|ext| path_lower.ends_with(ext))
}

/// エントリから音声ファイル情報（URL とファイルサイズ）を抽出する
///
/// feed-rs が `<media:player>` の URL で `<media:content>` の URL を上書きする問題を回避するため、
/// URL パスに音声ファイル拡張子を持つエントリを優先選択する。
fn find_audio_content(entry: &Entry) -> Option<(String, Option<u64>)> {
    let audio_contents: Vec<_> = entry
        .media
        .iter()
        .flat_map(|m| m.content.iter())
        .filter(|c| {
            c.url.is_some()
                && c.content_type
                    .as_ref()
                    .is_some_and(|mt| mt.to_string().starts_with("audio/"))
        })
        .collect();

    // audio/* かつ URL パスが音声拡張子を持つエントリを優先
    // （enclosure 由来のエントリは media:player による上書きが発生しない）
    for content in &audio_contents {
        if let Some(url) = &content.url {
            if has_audio_extension(url.path()) {
                return Some((url.to_string(), content.size));
            }
        }
    }

    // フォールバック: audio/* の最初のエントリ
    audio_contents.first().and_then(|c| {
        c.url.as_ref().map(|u| (u.to_string(), c.size))
    })
}

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
            // 音声ファイルの URL とサイズを取得
            let (audio_url, file_size) = {
                if let Some((url, size)) = find_audio_content(&entry) {
                    (url, size.map(|s| s as i64))
                } else if let Some(link) = entry
                    .links
                    .iter()
                    .find(|l| {
                        l.media_type
                            .as_deref()
                            .is_some_and(|mt| mt.starts_with("audio/"))
                    })
                {
                    (link.href.clone(), link.length.map(|s| s as i64))
                } else {
                    return None;
                }
            };

            let published_at = entry
                .published
                .or(entry.updated)
                .map(|dt| dt.to_rfc3339())?;

            Some(NewEpisode {
                guid: entry.id,
                title: entry.title.map(|t| t.content).unwrap_or_default(),
                description: entry.summary.map(|s| s.content),
                audio_url,
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
    fn test_episode_audio_prefers_enclosure_over_media_player() {
        // Omny FM 等のフィードでは <media:content> 内に <media:player> が入れ子になっており、
        // feed-rs が media:player の URL で media:content の URL を上書きしてしまう。
        // enclosure 由来のエントリが正しい音声 URL を持つため、そちらを優先する。
        let xml = minimal_rss(
            r#"<item>
              <guid>ep1</guid>
              <title>Episode 1</title>
              <media:content url="https://traffic.omny.fm/d/clips/audio.mp3?utm_source=Podcast" type="audio/mpeg">
                <media:player url="https://omny.fm/shows/my-show/ep1/embed" />
              </media:content>
              <media:content url="https://www.omnycontent.com/d/clips/image.jpg?size=Large" type="image/jpeg" />
              <enclosure url="https://traffic.omny.fm/d/clips/audio.mp3?utm_source=Podcast" type="audio/mpeg" length="50271640"/>
              <pubDate>Mon, 01 Jan 2024 00:00:00 GMT</pubDate>
            </item>"#,
        );

        let result = parse_feed(xml.as_bytes()).unwrap();
        assert_eq!(result.episodes.len(), 1);
        // enclosure の音声 URL が選択され、media:player の URL は使われないこと
        assert!(
            result.episodes[0].audio_url.contains("audio.mp3"),
            "Expected audio URL with .mp3, got: {}",
            result.episodes[0].audio_url,
        );
        assert!(
            !result.episodes[0].audio_url.contains("/embed"),
            "audio_url should not be the embed player URL: {}",
            result.episodes[0].audio_url,
        );
        assert_eq!(result.episodes[0].file_size, Some(50271640));
    }

    #[test]
    fn test_episode_audio_from_media_content_without_player() {
        // media:player がない場合は media:content の URL をそのまま使用する
        let xml = minimal_rss(
            r#"<item>
              <guid>ep1</guid>
              <title>Episode 1</title>
              <media:content url="https://example.com/audio.mp3" type="audio/mpeg" />
              <pubDate>Mon, 01 Jan 2024 00:00:00 GMT</pubDate>
            </item>"#,
        );

        let result = parse_feed(xml.as_bytes()).unwrap();
        assert_eq!(result.episodes.len(), 1);
        assert_eq!(result.episodes[0].audio_url, "https://example.com/audio.mp3");
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
