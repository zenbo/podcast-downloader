use tauri::ipc::Channel;
use tauri::{AppHandle, State};

use crate::commands::settings::load_settings;
use crate::db::{self, DbState};
use crate::error::AppError;
use crate::models::episode::{BatchDownloadProgress, DownloadProgress};
use crate::services::{downloader, filename};

/// エピソードをダウンロードする（進捗通知付き）
#[tauri::command]
pub async fn download_episode(
    episode_id: i64,
    on_progress: Channel<DownloadProgress>,
    state: State<'_, DbState>,
    app_handle: AppHandle,
) -> Result<(), AppError> {
    // 1. 設定を取得
    let settings = load_settings(&app_handle)?;
    let download_dir = settings
        .download_dir
        .ok_or_else(|| AppError::Other("ダウンロード先フォルダが設定されていません".to_string()))?;

    // 2. エピソードと番組情報を取得
    let (episode, podcast_title) = {
        let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
        let episode = db::episode::get(&conn, episode_id)?;
        let podcast = db::podcast::get(&conn, episode.podcast_id)?;
        (episode, podcast.title)
    };

    // 3. ダウンロード先パスを生成
    let save_path = filename::build_download_path(
        &download_dir,
        &podcast_title,
        &episode.title,
        &episode.published_at,
        &episode.audio_url,
        &settings.character_replacements,
        &settings.fallback_replacement,
    );

    // 4. ダウンロード実行
    downloader::download(&episode.audio_url, &save_path, episode_id, &on_progress).await?;

    // 5. DL 完了を記録
    let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
    db::episode::mark_downloaded(&conn, episode_id)?;

    Ok(())
}

/// 選択番組の新着エピソードを一括ダウンロードする
#[tauri::command]
pub async fn batch_download_new(
    podcast_ids: Vec<i64>,
    on_progress: Channel<BatchDownloadProgress>,
    state: State<'_, DbState>,
    app_handle: AppHandle,
) -> Result<(), AppError> {
    let settings = load_settings(&app_handle)?;
    let download_dir = settings
        .download_dir
        .ok_or_else(|| AppError::Other("ダウンロード先フォルダが設定されていません".to_string()))?;

    // 全番組の新着エピソードを収集
    let mut all_episodes: Vec<(crate::models::episode::Episode, String)> = Vec::new();
    for &podcast_id in &podcast_ids {
        let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
        let podcast = db::podcast::get(&conn, podcast_id)?;
        let new_episodes = db::episode::get_new_episodes(&conn, podcast_id)?;
        for ep in new_episodes {
            all_episodes.push((ep, podcast.title.clone()));
        }
    }

    let total_count = all_episodes.len();

    // 逐次ダウンロード
    for (idx, (episode, podcast_title)) in all_episodes.iter().enumerate() {
        let save_path = filename::build_download_path(
            &download_dir,
            podcast_title,
            &episode.title,
            &episode.published_at,
            &episode.audio_url,
            &settings.character_replacements,
            &settings.fallback_replacement,
        );

        // 個別エピソードの進捗を BatchDownloadProgress でラップして通知
        let episode_id = episode.id;
        let episode_title = episode.title.clone();

        // 内部進捗チャネルを使わず、chunk ごとに batch 進捗を通知
        // downloader は Channel<DownloadProgress> を要求するが、
        // batch では BatchDownloadProgress を送りたいので、
        // ここでは直接ダウンロード処理を行う
        let mut response = reqwest::get(&episode.audio_url).await.map_err(AppError::Http)?;
        let total_bytes = response.content_length();

        if let Some(parent) = save_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(AppError::FileSystem)?;
        }

        let mut file = tokio::fs::File::create(&save_path)
            .await
            .map_err(AppError::FileSystem)?;
        let mut downloaded_bytes: u64 = 0;

        use tokio::io::AsyncWriteExt;
        while let Some(chunk) = response.chunk().await.map_err(AppError::Http)? {
            file.write_all(&chunk).await.map_err(AppError::FileSystem)?;
            downloaded_bytes += chunk.len() as u64;

            let percentage = total_bytes.map(|total| {
                if total == 0 {
                    100.0
                } else {
                    (downloaded_bytes as f64 / total as f64) * 100.0
                }
            });

            let _ = on_progress.send(BatchDownloadProgress {
                current_episode_id: episode_id,
                current_episode_title: episode_title.clone(),
                episode_progress: DownloadProgress {
                    episode_id,
                    downloaded_bytes,
                    total_bytes,
                    percentage,
                },
                completed_count: idx,
                total_count,
            });
        }

        file.flush().await.map_err(AppError::FileSystem)?;

        // DL 完了を記録
        let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
        db::episode::mark_downloaded(&conn, episode_id)?;
    }

    Ok(())
}
