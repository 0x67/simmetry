mod app_state;
mod commands;
mod config;
mod error;
mod eval;
mod global;
mod logger;
mod setup;
mod ws_client;

#[macro_use]
extern crate tracing;

use crate::commands::{
    create_udp_listener::cmd_create_udp_listener, stop_udp_listener::cmd_stop_udp_listener,
};
use crate::logger::{debug, error, info, trace, warn};
use config::get_env;
use setup::{setup_app, setup_logging};
use std::env;
use tauri_plugin_sentry::{minidump, sentry};

use tauri::{Builder, Emitter, Manager};

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    setup_logging();
    let env = get_env();

    let client = sentry::init((
        env.sentry_dsn.clone().to_string(),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            auto_session_tracking: true,
            ..Default::default()
        },
    ));

    // Caution! Everything before here runs in both app and crash reporter processes
    #[cfg(not(target_os = "ios"))]
    let _guard = minidump::init(&client);

    tracing::info!("Operating System: {}", env::consts::OS);
    tracing::info!("OS Version: {}", os_info::get().version());
    tracing::info!("Architecture: {}", env::consts::ARCH);

    Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            if let Some(w) = app.get_webview_window("main") {
                info!("Application instance already running, focusing existing window");
                w.set_focus().unwrap();
            }
        }))
        .plugin(tauri_plugin_pinia::init())
        .plugin(tauri_plugin_keep_screen_on::init())
        .plugin(tauri_plugin_clipboard::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs_pro::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(setup_app)
        .invoke_handler(tauri::generate_handler![
            debug,
            error,
            info,
            trace,
            warn,
            cmd_create_udp_listener,
            cmd_stop_udp_listener,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
