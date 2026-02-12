mod api;
mod auth;
mod commands;
mod db;
mod models;

use std::sync::Mutex;

use commands::DbState;
use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

fn toggle_floating_memo(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("floating-memo") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    } else {
        let _ = tauri::WebviewWindowBuilder::new(
            app,
            "floating-memo",
            tauri::WebviewUrl::App("/floating-memo".into()),
        )
        .title("Quick Malt")
        .inner_size(480.0, 360.0)
        .always_on_top(true)
        .decorations(false)
        .center()
        .skip_taskbar(true)
        .build();
    }
}

struct DbusInterface {
    app: tauri::AppHandle,
}

#[zbus::interface(name = "com.distillery.App")]
impl DbusInterface {
    fn toggle_floating_memo(&self) {
        toggle_floating_memo(&self.app);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data dir");
            let conn = db::init_db(&app_data_dir);
            app.manage(DbState(Mutex::new(conn)));

            // Global shortcut (X11 only, silently fails on Wayland)
            let shortcut = if cfg!(target_os = "macos") {
                Shortcut::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyM)
            } else {
                Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyM)
            };

            let _ = app.global_shortcut().on_shortcut(shortcut, {
                let handle = app.handle().clone();
                move |_app, _shortcut, event| {
                    if event.state == ShortcutState::Pressed {
                        toggle_floating_memo(&handle);
                    }
                }
            });

            // DBus service for Wayland (user binds DE shortcut to dbus-send command)
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let iface = DbusInterface { app: handle };
                let conn = zbus::connection::Builder::session()
                    .expect("failed to create DBus session builder")
                    .name("com.distillery.App")
                    .expect("failed to set DBus name")
                    .serve_at("/com/distillery/App", iface)
                    .expect("failed to serve DBus interface")
                    .build()
                    .await
                    .expect("failed to build DBus connection");
                // Keep connection alive
                let _conn = conn;
                std::future::pending::<()>().await;
            });

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
            commands::hide_floating_memo,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
