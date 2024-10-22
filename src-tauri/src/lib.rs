use std::{
    collections::HashMap,
    sync::Arc,
};
use tokio::sync::Mutex;

mod cmd;
mod config;
mod runner;

pub struct AppState {
    config: config::ConfigFile,
    runners: Arc<Mutex<HashMap<String, runner::RunnerClient>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let r = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            host_platform,
            cmd::runner::runner_info,
        ])
        .run(tauri::generate_context!());

    match r {
        Ok(_) => {}
        Err(e) => {
            eprintln!("An error occurred while Volkanic Console was running: {}", e);
        }
    }
}

/// Returns the platform compiled for
#[tauri::command]
async fn host_platform() -> String {
    #[cfg(target_os = "windows")]
    {"windows".to_string()}
    #[cfg(target_os = "macos")]
    {"macos".to_string()}
    #[cfg(target_os = "linux")]
    {"linux".to_string()}
}
