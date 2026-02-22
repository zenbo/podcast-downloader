mod commands;
mod db;
mod error;
mod models;
mod services;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|_app| {
            // DB 初期化やマイグレーションは後のフェーズで実装
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
