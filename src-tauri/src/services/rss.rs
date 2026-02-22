use feed_rs::parser;

use crate::error::AppError;
use crate::models::episode::NewEpisode;
use crate::models::podcast::PodcastFeed;

/// RSS フィードを取得・パースし、PodcastFeed として返す
pub async fn fetch_and_parse(feed_url: &str) -> Result<PodcastFeed, AppError> {
    let body = reqwest::get(feed_url)
        .await?
        .bytes()
        .await?;

    let feed = parser::parse(&body[..])
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
