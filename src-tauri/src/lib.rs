mod api;
mod auth;
mod commands;
mod db;
mod models;

use std::sync::Mutex;

use commands::DbState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data dir");
            let conn = db::init_db(&app_data_dir);
            app.manage(DbState(Mutex::new(conn)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_malts_by_status,
            commands::add_malt,
            commands::delete_malt,
            commands::update_malt,
            commands::set_malt_status,
            commands::search_malts,
            commands::auth_send_otp,
            commands::auth_verify_otp,
            commands::auth_check,
            commands::auth_logout,
            commands::get_queued_malts,
            commands::malt_draw_back,
            commands::malt_queue,
            commands::malts_queue_batch,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
