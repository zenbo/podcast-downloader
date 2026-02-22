use std::sync::Mutex;

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
        .setup(|app| {
            use tauri::Manager;
            let app_data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data_dir)?;
            let conn = db::init_db(&app_data_dir)?;
            app.manage(db::DbState(Mutex::new(conn)));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
