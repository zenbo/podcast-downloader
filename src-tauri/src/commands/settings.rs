use tauri::AppHandle;
use tauri::State;
use tauri_plugin_dialog::DialogExt;

use crate::error::AppError;
use crate::models::settings::AppSettings;
use crate::services::traits::ServiceContainer;

/// 設定を取得する
#[tauri::command]
pub async fn get_settings(services: State<'_, ServiceContainer>) -> Result<AppSettings, AppError> {
    services.settings_store.load_settings()
}

/// 設定を保存する
#[tauri::command]
pub async fn update_settings(
    settings: AppSettings,
    services: State<'_, ServiceContainer>,
) -> Result<(), AppError> {
    services.settings_store.save_settings(&settings)
}

/// フォルダ選択ダイアログを表示する
#[tauri::command]
pub async fn select_folder(app_handle: AppHandle) -> Result<Option<String>, AppError> {
    let path = app_handle.dialog().file().blocking_pick_folder();
    Ok(path.map(|p| p.to_string()))
}
