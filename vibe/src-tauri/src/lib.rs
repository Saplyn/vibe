use chrono::Local;
use log::{Level, LevelFilter};
use owo_colors::OwoColorize;

use crate::build::print_built_info;

mod build;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    print_built_info();

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                .level_for("vibe", LevelFilter::Trace)
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "{} {} {} {}",
                        Local::now().bright_black(),
                        match record.level() {
                            Level::Error => record.level().bright_red().to_string(),
                            Level::Warn => record.level().bright_yellow().to_string(),
                            Level::Info => record.level().bright_green().to_string(),
                            Level::Debug => record.level().bright_cyan().to_string(),
                            Level::Trace => record.level().bright_black().to_string(),
                        },
                        record.target().bright_black(),
                        message
                    ))
                })
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
