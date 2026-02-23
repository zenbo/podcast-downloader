use std::path::PathBuf;

use rusqlite::Connection;
use tauri::ipc::Channel;
use tauri::State;

use crate::db::{self, DbState};
use crate::error::AppError;
use crate::models::episode::{BatchDownloadProgress, DownloadProgress, Episode};
use crate::services::filename;
use crate::services::traits::{FileDownloader, ServiceContainer};

/// エピソードをダウンロードする（進捗通知付き）
#[tauri::command]
pub async fn download_episode(
    episode_id: i64,
    on_progress: Channel<DownloadProgress>,
    state: State<'_, DbState>,
    services: State<'_, ServiceContainer>,
) -> Result<(), AppError> {
    let settings = services.settings_store.load_settings()?;
    let download_dir = settings
        .download_dir
        .as_deref()
        .ok_or_else(|| AppError::Other("ダウンロード先フォルダが設定されていません".to_string()))?
        .to_string();

    let (episode, podcast_title) = {
        let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
        let episode = db::episode::get(&conn, episode_id)?;
        let podcast = db::podcast::get(&conn, episode.podcast_id)?;
        (episode, podcast.title)
    };

    let save_path = filename::build_download_path(
        &download_dir,
        &podcast_title,
        &episode.title,
        &episode.published_at,
        &episode.audio_url,
        &settings.character_replacements,
        &settings.fallback_replacement,
    );

    download_episode_impl(
        &*services.file_downloader,
        &episode,
        &save_path,
        Box::new(move |progress| {
            let _ = on_progress.send(progress);
        }),
    )
    .await?;

    let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
    db::episode::mark_downloaded(&conn, episode_id)?;

    Ok(())
}

/// download_episode のテスト可能なロジック部分
pub(crate) async fn download_episode_impl(
    downloader: &dyn FileDownloader,
    episode: &Episode,
    save_path: &PathBuf,
    on_progress: Box<dyn FnMut(DownloadProgress) + Send>,
) -> Result<(), AppError> {
    downloader
        .download(&episode.audio_url, save_path, episode.id, on_progress)
        .await
}

/// 選択番組の新着エピソードを一括ダウンロードする
#[tauri::command]
pub async fn batch_download_new(
    podcast_ids: Vec<i64>,
    on_progress: Channel<BatchDownloadProgress>,
    state: State<'_, DbState>,
    services: State<'_, ServiceContainer>,
) -> Result<(), AppError> {
    let settings = services.settings_store.load_settings()?;
    let download_dir = settings
        .download_dir
        .as_deref()
        .ok_or_else(|| AppError::Other("ダウンロード先フォルダが設定されていません".to_string()))?
        .to_string();

    // 全番組の新着エピソードを収集
    let all_episodes = {
        let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
        collect_new_episodes(&conn, &podcast_ids)?
    };

    let total_count = all_episodes.len();

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

        let episode_id = episode.id;
        let episode_title = episode.title.clone();
        let progress_channel = on_progress.clone();

        services
            .file_downloader
            .download(
                &episode.audio_url,
                &save_path,
                episode_id,
                Box::new(move |progress| {
                    let _ = progress_channel.send(BatchDownloadProgress {
                        current_episode_id: episode_id,
                        current_episode_title: episode_title.clone(),
                        episode_progress: progress,
                        completed_count: idx,
                        total_count,
                    });
                }),
            )
            .await?;

        let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
        db::episode::mark_downloaded(&conn, episode_id)?;
    }

    Ok(())
}

/// 指定番組の新着エピソードを収集する（テスト可能なヘルパー）
pub(crate) fn collect_new_episodes(
    conn: &Connection,
    podcast_ids: &[i64],
) -> Result<Vec<(Episode, String)>, AppError> {
    let mut all_episodes = Vec::new();
    for &podcast_id in podcast_ids {
        let podcast = db::podcast::get(conn, podcast_id)?;
        let new_episodes = db::episode::get_new_episodes(conn, podcast_id)?;
        for ep in new_episodes {
            all_episodes.push((ep, podcast.title.clone()));
        }
    }
    Ok(all_episodes)
}

