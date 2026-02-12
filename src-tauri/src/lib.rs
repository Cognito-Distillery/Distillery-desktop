mod api;
mod auth;
mod commands;
mod db;
mod models;

use std::sync::Mutex;

use commands::DbState;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::TrayIconBuilder;
use tauri::{Manager, WindowEvent};
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

fn show_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
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

            // --- Tray icon ---
            let show = MenuItemBuilder::with_id("show", "Show Distillery").build(app)?;
            let floating = MenuItemBuilder::with_id("floating", "Quick Malt").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let menu = MenuBuilder::new(app)
                .item(&show)
                .item(&floating)
                .separator()
                .item(&quit)
                .build()?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("Distillery")
                .menu(&menu)
                .on_menu_event({
                    let handle = app.handle().clone();
                    move |_tray, event| match event.id().as_ref() {
                        "show" => show_main_window(&handle),
                        "floating" => toggle_floating_memo(&handle),
                        "quit" => {
                            handle.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event({
                    let handle = app.handle().clone();
                    move |_tray, event| {
                        if let tauri::tray::TrayIconEvent::Click { button, .. } = event {
                            if button == tauri::tray::MouseButton::Left {
                                show_main_window(&handle);
                            }
                        }
                    }
                })
                .build(app)?;

            // --- Global shortcut (X11 only, silently fails on Wayland) ---
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

            // --- DBus service for Wayland ---
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
                let _conn = conn;
                std::future::pending::<()>().await;
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            // Hide main window on close instead of quitting
            if window.label() == "main" {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
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
