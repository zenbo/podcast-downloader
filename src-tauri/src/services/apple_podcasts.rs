use std::sync::LazyLock;

use regex::Regex;
use serde::Deserialize;

use crate::error::AppError;

static PODCAST_ID_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"id(\d+)").unwrap());

#[derive(Deserialize)]
struct ItunesResponse {
    results: Vec<ItunesResult>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ItunesResult {
    feed_url: Option<String>,
}

/// Apple Podcasts URL から Podcast ID を抽出する
pub fn extract_podcast_id(url: &str) -> Result<String, AppError> {
    let caps = PODCAST_ID_RE
        .captures(url)
        .ok_or_else(|| AppError::PodcastIdNotFound(url.to_string()))?;
    Ok(caps[1].to_string())
}

/// Apple Podcasts URL から RSS フィード URL を解決する
///
/// 1. URL から Podcast ID を抽出
/// 2. iTunes Lookup API で feedUrl を取得
pub async fn resolve_feed_url(url: &str) -> Result<String, AppError> {
    let podcast_id = extract_podcast_id(url)?;
    let api_url = format!(
        "https://itunes.apple.com/lookup?id={}&entity=podcast",
        podcast_id
    );

    let response: ItunesResponse = reqwest::get(&api_url).await?.json().await?;

    let feed_url = response
        .results
        .into_iter()
        .find_map(|r| r.feed_url)
        .ok_or(AppError::FeedUrlNotFound)?;

    Ok(feed_url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_podcast_id_standard_url() {
        let url = "https://podcasts.apple.com/us/podcast/some-podcast/id1234567890";
        assert_eq!(extract_podcast_id(url).unwrap(), "1234567890");
    }

    #[test]
    fn test_extract_podcast_id_jp_url() {
        let url = "https://podcasts.apple.com/jp/podcast/ポッドキャスト名/id9876543";
        assert_eq!(extract_podcast_id(url).unwrap(), "9876543");
    }

    #[test]
    fn test_extract_podcast_id_with_query_params() {
        let url = "https://podcasts.apple.com/us/podcast/name/id12345?mt=2&ls=1";
        assert_eq!(extract_podcast_id(url).unwrap(), "12345");
    }

    #[test]
    fn test_extract_podcast_id_invalid_url() {
        let url = "https://example.com/no-id-here";
        assert!(extract_podcast_id(url).is_err());
    }
}
