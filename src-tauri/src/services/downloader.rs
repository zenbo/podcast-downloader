use std::path::Path;

use tokio::io::AsyncWriteExt;

use crate::error::AppError;
use crate::models::episode::DownloadProgress;

/// 音声ファイルを HTTP ストリーミングでダウンロードし、進捗をコールバックで通知する
pub async fn download(
    audio_url: &str,
    save_path: &Path,
    episode_id: i64,
    mut on_progress: impl FnMut(DownloadProgress),
) -> Result<(), AppError> {
    // 保存先ディレクトリを作成
    if let Some(parent) = save_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    let mut response = reqwest::get(audio_url).await?;

    if !response.status().is_success() {
        return Err(AppError::Other(format!(
            "ダウンロード失敗: HTTP {} (URL: {})",
            response.status(),
            audio_url,
        )));
    }

    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if !content_type.is_empty()
        && !content_type.starts_with("audio/")
        && !content_type.starts_with("application/octet-stream")
    {
        log::warn!(
            "予期しない Content-Type: {} (URL: {})",
            content_type,
            audio_url,
        );
    }

    let total_bytes = response.content_length();

    let mut file = tokio::fs::File::create(save_path).await?;
    let mut downloaded_bytes: u64 = 0;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
        downloaded_bytes += chunk.len() as u64;

        let percentage = total_bytes.map(|total| {
            if total == 0 {
                100.0
            } else {
                (downloaded_bytes as f64 / total as f64) * 100.0
            }
        });

        on_progress(DownloadProgress {
            episode_id,
            downloaded_bytes,
            total_bytes,
            percentage,
        });
    }

    file.flush().await?;
    Ok(())
}
