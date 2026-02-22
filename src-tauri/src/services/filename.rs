use std::path::{Path, PathBuf};

use crate::models::settings::CharacterReplacement;

/// Windows の OS 禁止文字
const FORBIDDEN_CHARS: &[char] = &['\\', '/', ':', '*', '?', '"', '<', '>', '|'];

/// 文字置換ルールを適用し、OS 禁止文字をフォールバック文字で置換する
pub fn sanitize(
    text: &str,
    replacements: &[CharacterReplacement],
    fallback: &str,
) -> String {
    let mut result = text.to_string();

    // 1. ユーザー定義の置換ルールを順序どおり適用
    for rule in replacements {
        result = result.replace(&rule.before, &rule.after);
    }

    // 2. 残存する OS 禁止文字をフォールバック文字で置換
    for &ch in FORBIDDEN_CHARS {
        result = result.replace(ch, fallback);
    }

    // 3. 前後の空白を除去
    result.trim().to_string()
}

/// ダウンロード先のファイルパスを生成する
///
/// 形式: `{download_dir}/{sanitized_podcast_title}/{YYYY-MM-DD}_{sanitized_episode_title}.{ext}`
pub fn build_download_path(
    download_dir: &str,
    podcast_title: &str,
    episode_title: &str,
    published_at: &str,
    audio_url: &str,
    replacements: &[CharacterReplacement],
    fallback: &str,
) -> PathBuf {
    let sanitized_podcast = sanitize(podcast_title, replacements, fallback);
    let sanitized_episode = sanitize(episode_title, replacements, fallback);

    // published_at ("2026-02-22T10:30:00Z") から日付部分を抽出
    let date_part = published_at
        .split('T')
        .next()
        .unwrap_or("0000-00-00");

    // 音声 URL から拡張子を抽出
    let ext = extract_extension(audio_url);

    let filename = format!("{}_{}.{}", date_part, sanitized_episode, ext);

    PathBuf::from(download_dir)
        .join(sanitized_podcast)
        .join(filename)
}

/// URL から拡張子を抽出する（クエリパラメータを除去）
fn extract_extension(url: &str) -> &str {
    // クエリパラメータやフラグメントを除去
    let path = url.split('?').next().unwrap_or(url);
    let path = path.split('#').next().unwrap_or(path);

    Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("mp3")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_replacements() -> Vec<CharacterReplacement> {
        vec![
            CharacterReplacement { before: "/".to_string(), after: "-".to_string() },
            CharacterReplacement { before: ":".to_string(), after: "-".to_string() },
            CharacterReplacement { before: "?".to_string(), after: "".to_string() },
            CharacterReplacement { before: "\"".to_string(), after: "".to_string() },
            CharacterReplacement { before: "<".to_string(), after: "".to_string() },
            CharacterReplacement { before: ">".to_string(), after: "".to_string() },
            CharacterReplacement { before: "|".to_string(), after: "".to_string() },
        ]
    }

    #[test]
    fn test_sanitize_basic() {
        let result = sanitize("Hello: World", &default_replacements(), "_");
        assert_eq!(result, "Hello- World");
    }

    #[test]
    fn test_sanitize_multiple_replacements() {
        let result = sanitize("A/B:C?D", &default_replacements(), "_");
        assert_eq!(result, "A-B-CD");
    }

    #[test]
    fn test_sanitize_fallback_for_remaining_forbidden() {
        // * は個別ルールにないので fallback で置換
        let result = sanitize("file*name", &default_replacements(), "_");
        assert_eq!(result, "file_name");
    }

    #[test]
    fn test_sanitize_no_rules() {
        let result = sanitize("Hello: World", &[], "_");
        assert_eq!(result, "Hello_ World");
    }

    #[test]
    fn test_sanitize_trims_whitespace() {
        let result = sanitize("  hello  ", &[], "_");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_build_download_path() {
        let path = build_download_path(
            "/Users/test/Podcasts",
            "My Podcast: Special",
            "Episode 1: Introduction",
            "2026-02-22T10:30:00Z",
            "https://example.com/ep1.mp3",
            &default_replacements(),
            "_",
        );
        assert_eq!(
            path,
            PathBuf::from("/Users/test/Podcasts/My Podcast- Special/2026-02-22_Episode 1- Introduction.mp3")
        );
    }

    #[test]
    fn test_build_download_path_with_query_url() {
        let path = build_download_path(
            "C:\\Podcasts",
            "Show",
            "Ep 1",
            "2026-01-01T00:00:00Z",
            "https://cdn.example.com/audio.m4a?token=abc123",
            &[],
            "_",
        );
        let filename = path.file_name().unwrap().to_str().unwrap();
        assert!(filename.ends_with(".m4a"));
    }

    #[test]
    fn test_extract_extension() {
        assert_eq!(extract_extension("https://example.com/file.mp3"), "mp3");
        assert_eq!(extract_extension("https://example.com/file.m4a?key=val"), "m4a");
        assert_eq!(extract_extension("https://example.com/file.ogg#anchor"), "ogg");
    }
}
