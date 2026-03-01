use rusqlite::Connection;
use tauri::ipc::Channel;
use tauri::State;

use crate::db::{self, DbState};
use crate::error::AppError;
use crate::models::episode::{
    BatchDownloadProgress, BatchDownloadSummary, DownloadProgress, Episode,
};
use crate::services::filename;
use crate::services::traits::{FileDownloader, ServiceContainer, SettingsStore};

/// エピソードをダウンロードする（進捗通知付き）
#[tauri::command]
pub async fn download_episode(
    episode_id: i64,
    on_progress: Channel<DownloadProgress>,
    state: State<'_, DbState>,
    services: State<'_, ServiceContainer>,
) -> Result<(), AppError> {
    download_episode_workflow(
        &state,
        &*services.settings_store,
        &*services.file_downloader,
        episode_id,
        Box::new(move |progress| {
            let _ = on_progress.send(progress);
        }),
    )
    .await
}

/// download_episode のテスト可能なワークフロー全体
pub(crate) async fn download_episode_workflow(
    state: &DbState,
    settings_store: &dyn SettingsStore,
    file_downloader: &dyn FileDownloader,
    episode_id: i64,
    on_progress: Box<dyn FnMut(DownloadProgress) + Send>,
) -> Result<(), AppError> {
    let settings = settings_store.load_settings()?;
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

    file_downloader
        .download(&episode.audio_url, &save_path, episode.id, on_progress)
        .await?;

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
    services: State<'_, ServiceContainer>,
) -> Result<BatchDownloadSummary, AppError> {
    let settings = services.settings_store.load_settings()?;
    let download_dir = settings
        .download_dir
        .as_deref()
        .ok_or_else(|| AppError::Other("ダウンロード先フォルダが設定されていません".to_string()))?
        .to_string();

    let all_episodes = {
        let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
        collect_new_episodes(&conn, &podcast_ids)?
    };

    let total_count = all_episodes.len();
    log::info!(
        "batch_download_new: podcast_ids={:?}, 対象エピソード数={}",
        podcast_ids,
        total_count
    );
    let mut completed_count: usize = 0;
    let mut failed_count: usize = 0;

    for (episode, podcast_title) in &all_episodes {
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
        let current_completed = completed_count;

        let download_result = services
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
                        completed_count: current_completed,
                        total_count,
                    });
                }),
            )
            .await;

        match download_result {
            Ok(()) => {
                let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
                db::episode::mark_downloaded(&conn, episode_id)?;
                completed_count += 1;
            }
            Err(e) => {
                log::warn!(
                    "エピソード「{}」のダウンロードに失敗、スキップ: {}",
                    episode.title,
                    e
                );
                failed_count += 1;
            }
        }
    }

    if failed_count > 0 {
        log::warn!(
            "一括ダウンロード完了: {}/{} 件成功、{} 件失敗",
            completed_count,
            total_count,
            failed_count,
        );
    }

    Ok(BatchDownloadSummary {
        completed_count,
        failed_count,
        total_count,
    })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::test_helpers::*;
    use crate::db;
    use crate::services::traits::{FileDownloader, SettingsStore};

    /// 一括ダウンロードの結果
    #[derive(Debug, Clone)]
    struct BatchDownloadResult {
        completed_count: usize,
        failed_count: usize,
        total_count: usize,
    }

    /// batch_download_new のテスト可能なワークフロー全体
    ///
    /// 個別エピソードのダウンロード失敗時はスキップして次のエピソードへ進む（部分的成功パターン）。
    async fn batch_download_new_workflow(
        state: &DbState,
        settings_store: &dyn SettingsStore,
        file_downloader: &dyn FileDownloader,
        podcast_ids: &[i64],
    ) -> Result<BatchDownloadResult, AppError> {
        let settings = settings_store.load_settings()?;
        let download_dir = settings
            .download_dir
            .as_deref()
            .ok_or_else(|| {
                AppError::Other("ダウンロード先フォルダが設定されていません".to_string())
            })?
            .to_string();

        let all_episodes = {
            let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
            collect_new_episodes(&conn, podcast_ids)?
        };

        let total_count = all_episodes.len();
        let mut completed_count: usize = 0;
        let mut failed_count: usize = 0;

        for (episode, podcast_title) in &all_episodes {
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

            let download_result = file_downloader
                .download(&episode.audio_url, &save_path, episode_id, Box::new(|_| {}))
                .await;

            match download_result {
                Ok(()) => {
                    let conn = state.0.lock().map_err(|e| AppError::Other(e.to_string()))?;
                    db::episode::mark_downloaded(&conn, episode_id)?;
                    completed_count += 1;
                }
                Err(e) => {
                    log::warn!(
                        "エピソード「{}」のダウンロードに失敗、スキップ: {}",
                        episode.title,
                        e
                    );
                    failed_count += 1;
                }
            }
        }

        Ok(BatchDownloadResult {
            completed_count,
            failed_count,
            total_count,
        })
    }

    #[tokio::test]
    async fn test_download_episode_settings_not_configured() {
        let state = create_test_db_state();
        let pid = setup_podcast_in_state(&state, "Podcast", "https://example.com/feed");
        let episodes = make_episodes(1);
        setup_episodes_in_state(&state, pid, &episodes);

        let episode_id = {
            let conn = state.0.lock().unwrap();
            db::episode::list_by_podcast(&conn, pid).unwrap()[0].id
        };

        let settings_store = MockSettingsStore::without_download_dir();
        let downloader = MockFileDownloader::always_ok();

        let result = download_episode_workflow(
            &state,
            &settings_store,
            &downloader,
            episode_id,
            Box::new(|_| {}),
        )
        .await;

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("ダウンロード先フォルダが設定されていません"));

        // ダウンローダーが呼ばれていないことを確認
        assert_eq!(downloader.get_call_count(), 0);
    }

    #[tokio::test]
    async fn test_download_episode_success_marks_downloaded() {
        let state = create_test_db_state();
        let pid = setup_podcast_in_state(&state, "Podcast", "https://example.com/feed");
        let episodes = make_episodes(1);
        setup_episodes_in_state(&state, pid, &episodes);

        let episode_id = {
            let conn = state.0.lock().unwrap();
            db::episode::list_by_podcast(&conn, pid).unwrap()[0].id
        };

        let settings_store = MockSettingsStore::with_download_dir("/tmp/podcasts");
        let downloader = MockFileDownloader::always_ok();

        let result = download_episode_workflow(
            &state,
            &settings_store,
            &downloader,
            episode_id,
            Box::new(|_| {}),
        )
        .await;

        assert!(result.is_ok());

        // downloaded_at が設定されたことを確認
        let conn = state.0.lock().unwrap();
        let ep = db::episode::get(&conn, episode_id).unwrap();
        assert!(ep.downloaded_at.is_some());
    }

    #[tokio::test]
    async fn test_download_episode_failure_does_not_mark_downloaded() {
        let state = create_test_db_state();
        let pid = setup_podcast_in_state(&state, "Podcast", "https://example.com/feed");
        let episodes = make_episodes(1);
        setup_episodes_in_state(&state, pid, &episodes);

        let episode_id = {
            let conn = state.0.lock().unwrap();
            db::episode::list_by_podcast(&conn, pid).unwrap()[0].id
        };

        let settings_store = MockSettingsStore::with_download_dir("/tmp/podcasts");
        let downloader = MockFileDownloader::always_err("connection timeout");

        let result = download_episode_workflow(
            &state,
            &settings_store,
            &downloader,
            episode_id,
            Box::new(|_| {}),
        )
        .await;

        assert!(result.is_err());

        // downloaded_at が None のままであることを確認
        let conn = state.0.lock().unwrap();
        let ep = db::episode::get(&conn, episode_id).unwrap();
        assert!(ep.downloaded_at.is_none());
    }

    #[tokio::test]
    async fn test_batch_download_partial_success() {
        let state = create_test_db_state();
        let pid = setup_podcast_in_state(&state, "Podcast", "https://example.com/feed");
        let episodes = make_episodes(3);
        setup_episodes_in_state(&state, pid, &episodes);

        let settings_store = MockSettingsStore::with_download_dir("/tmp/podcasts");

        // 1番目: 成功、2番目: 失敗、3番目: 成功
        let downloader = MockFileDownloader::with_results(vec![
            Ok(()),
            Err("download failed".to_string()),
            Ok(()),
        ]);

        let result = batch_download_new_workflow(&state, &settings_store, &downloader, &[pid])
            .await
            .unwrap();

        assert_eq!(result.completed_count, 2);
        assert_eq!(result.failed_count, 1);
        assert_eq!(result.total_count, 3);

        // DL 済みエピソード数を確認
        let conn = state.0.lock().unwrap();
        let all_eps = db::episode::list_by_podcast(&conn, pid).unwrap();
        let downloaded_count = all_eps.iter().filter(|e| e.downloaded_at.is_some()).count();
        assert_eq!(downloaded_count, 2);
    }

    #[tokio::test]
    async fn test_batch_download_completed_count_accuracy() {
        let state = create_test_db_state();
        let pid = setup_podcast_in_state(&state, "Podcast", "https://example.com/feed");
        let episodes = make_episodes(3);
        setup_episodes_in_state(&state, pid, &episodes);

        let settings_store = MockSettingsStore::with_download_dir("/tmp/podcasts");
        let downloader = MockFileDownloader::always_ok();

        let result = batch_download_new_workflow(&state, &settings_store, &downloader, &[pid])
            .await
            .unwrap();

        assert_eq!(result.completed_count, 3);
        assert_eq!(result.failed_count, 0);
        assert_eq!(result.total_count, 3);
    }

    #[test]
    fn test_collect_new_episodes_returns_correct_pairs() {
        let state = create_test_db_state();
        let pid = setup_podcast_in_state(&state, "My Show", "https://example.com/feed");
        let episodes = make_episodes(3);
        setup_episodes_in_state(&state, pid, &episodes);

        let conn = state.0.lock().unwrap();
        let result = collect_new_episodes(&conn, &[pid]).unwrap();

        assert_eq!(result.len(), 3);
        for (_, title) in &result {
            assert_eq!(title, "My Show");
        }
    }

    #[test]
    fn test_collect_new_episodes_empty() {
        let state = create_test_db_state();
        let pid = setup_podcast_in_state(&state, "Empty Show", "https://example.com/feed");

        let conn = state.0.lock().unwrap();
        let result = collect_new_episodes(&conn, &[pid]).unwrap();

        assert!(result.is_empty());
    }
}
