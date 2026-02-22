use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_store::StoreExt;

use crate::error::AppError;
use crate::models::settings::AppSettings;

const STORE_FILENAME: &str = "settings.json";
const SETTINGS_KEY: &str = "settings";

/// tauri-plugin-store から設定を読み込むヘルパー
pub fn load_settings(app_handle: &AppHandle) -> Result<AppSettings, AppError> {
    let store = app_handle
        .store(STORE_FILENAME)
        .map_err(|e| AppError::Other(e.to_string()))?;

    let settings = store
        .get(SETTINGS_KEY)
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    Ok(settings)
}

/// 設定を取得する
#[tauri::command]
pub async fn get_settings(app_handle: AppHandle) -> Result<AppSettings, AppError> {
    load_settings(&app_handle)
}

/// 設定を保存する
#[tauri::command]
pub async fn update_settings(
    settings: AppSettings,
    app_handle: AppHandle,
) -> Result<(), AppError> {
    let store = app_handle
        .store(STORE_FILENAME)
        .map_err(|e| AppError::Other(e.to_string()))?;

    store.set(
        SETTINGS_KEY,
        serde_json::to_value(&settings).map_err(|e| AppError::Other(e.to_string()))?,
    );

    Ok(())
}

/// フォルダ選択ダイアログを表示する
#[tauri::command]
pub async fn select_folder(app_handle: AppHandle) -> Result<Option<String>, AppError> {
    let path = app_handle.dialog().file().blocking_pick_folder();
    Ok(path.map(|p| p.to_string()))
}
