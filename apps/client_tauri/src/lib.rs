mod app_state;
mod commands;
mod constants;
mod log;
mod setup;
mod ws_server;

use std::env;

use crate::app_state::AppState;
use crate::log::{debug, error, info, trace, warn};
use commands::{create_udp_listener::create_udp_listener, stop_udp_listener::stop_udp_listener};
use setup::setup_logging;
use tauri::Manager;
use tokio::sync::Mutex;

#[macro_use]
extern crate tracing;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Builder,
};

pub fn run() {
    // setup_logging();

    info!("Operating System: {}", env::consts::OS);
    info!("OS Version: {}", os_info::get().version());
    info!("Architecture: {}", env::consts::ARCH);

    info!("Initializing application");

    Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .show_menu_on_left_click(true)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    other => {
                        println!("menu item {} not handled", other);
                    }
                })
                .build(app)?;

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            trace,
            debug,
            info,
            warn,
            error,
            create_udp_listener,
            stop_udp_listener
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
