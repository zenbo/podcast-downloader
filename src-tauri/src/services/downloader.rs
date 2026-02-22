use std::path::Path;

use tauri::ipc::Channel;
use tokio::io::AsyncWriteExt;

use crate::error::AppError;
use crate::models::episode::DownloadProgress;

/// 音声ファイルを HTTP ストリーミングでダウンロードし、進捗を Channel で通知する
pub async fn download(
    audio_url: &str,
    save_path: &Path,
    episode_id: i64,
    on_progress: &Channel<DownloadProgress>,
) -> Result<(), AppError> {
    // 保存先ディレクトリを作成
    if let Some(parent) = save_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    let mut response = reqwest::get(audio_url).await?;
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

        let _ = on_progress.send(DownloadProgress {
            episode_id,
            downloaded_bytes,
            total_bytes,
            percentage,
        });
    }

    file.flush().await?;
    Ok(())
}
