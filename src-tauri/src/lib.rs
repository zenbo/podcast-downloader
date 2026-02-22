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
        .invoke_handler(tauri::generate_handler![
            commands::podcast::register_podcast,
            commands::podcast::list_podcasts,
            commands::podcast::delete_podcast,
            commands::episode::list_episodes,
            commands::episode::check_new_episodes,
            commands::episode::check_all_new,
            commands::download::download_episode,
            commands::download::batch_download_new,
            commands::settings::get_settings,
            commands::settings::update_settings,
            commands::settings::select_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
