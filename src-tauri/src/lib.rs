use std::{collections::HashMap, sync::Arc};
use tauri::Manager;
use tokio::sync::{broadcast, Mutex};

mod cmd;
mod config;
mod runner;

use config::ConfigFile;
use runner::event::RemoteEvent;

pub struct AppState {
    config: config::ConfigFile,
    runners: Arc<Mutex<HashMap<String, Arc<runner::Runner>>>>,
    event_listener: broadcast::Sender<RemoteEvent>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let config = match ConfigFile::new().await {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Failed to initiate config: {}", e);
            std::process::exit(1);
        }
    };

    let state = AppState {
        config,
        runners: Arc::new(Mutex::new(HashMap::new())),
        event_listener: broadcast::channel(4096).0,
    };

    let r = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            app.manage(state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            app_version,
            host_platform,
            cmd::instance::del_instance,
            cmd::instance::new_instance,
            cmd::misc::is_valid_url,
            cmd::runner::runner_info,
            cmd::runner::runner_list,
            cmd::runner::runner_new,
        ])
        .run(tauri::generate_context!());

    match r {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "An error occurred while Volkanic Console was running: {}",
                e
            );
            std::process::exit(1);
        }
    }
}

#[tauri::command]
async fn app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}


/// Returns the platform compiled for
#[tauri::command]
async fn host_platform() -> String {
    #[cfg(target_os = "windows")]
    {
        "windows".to_string()
    }
    #[cfg(target_os = "macos")]
    {
        "macos".to_string()
    }
    #[cfg(target_os = "linux")]
    {
        "linux".to_string()
    }
}
