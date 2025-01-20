mod window_event_handler;
mod setup_app;
mod constants;
mod abstractions;
mod mocks;
mod commands;
mod system_tray;
mod fight_simulator;
mod background;
mod misc;

use log::error;
use tauri::{generate_context, Context};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_log::{Target, TargetKind};
use commands::generate_handlers;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    std::panic::set_hook(Box::new(|info| {
        let payload = info.payload();
        let message = if let Some(s) = payload.downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = payload.downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic message".to_string()
        };

        let location = info.location().map_or("unknown location".to_string(), |location| {
            format!("{}:{}", location.file(), location.line())
        });

        error!("Panicked at '{}', {}", message, location);
    }));

    let handlers = generate_handlers();
    let context: Context = generate_context!();

    tauri::Builder::default()
            .plugin(tauri_plugin_log::Builder::new()
            .level(log::LevelFilter::Debug)
            .targets([
                Target::new(TargetKind::Stdout),
                Target::new(TargetKind::LogDir {
                    file_name: Some("logs".to_string()),
                })
            ])
            .build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        // .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .plugin(
            tauri_plugin_window_state::Builder::new()
                .skip_initial_state("main")
                .skip_initial_state("logs")
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .on_window_event(window_event_handler::on_window_event)
        .setup(setup_app::setup_app)
        .invoke_handler(handlers)
        .run(context)
        .expect("error while running tauri application");
}
