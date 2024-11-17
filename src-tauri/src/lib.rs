use std::{collections::HashMap, sync::Arc};
use tauri::Manager;
use tauri_plugin_sentry::sentry;
use tokio::sync::{broadcast, Mutex};
use tracing::{debug, error};

mod cmd;
mod config;
mod runner;

use config::ConfigFile;
use runner::event::RemoteEvent;

const DEBUG_MODE_VAR: &str = "VK_DEBUG";

pub struct AppState {
    config: config::ConfigFile,
    runners: Arc<Mutex<HashMap<String, Arc<runner::Runner>>>>,
    event_listener: broadcast::Sender<RemoteEvent>,
    sentry_guard: Arc<Mutex<Option<sentry::ClientInitGuard>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let debug_mode = std::env::var(DEBUG_MODE_VAR) == Ok(String::from("true"));

    tracing_subscriber::fmt()
        .with_max_level({match debug_mode {
            true => tracing::Level::DEBUG,
            false => tracing::Level::INFO,
        }})
        .init();

    let config = match ConfigFile::new().await {
        Ok(o) => o,
        Err(e) => {
            error!("Failed to initiate config: {}", e);
            std::process::exit(1);
        }
    };

    let state = AppState {
        config,
        runners: Arc::new(Mutex::new(HashMap::new())),
        event_listener: broadcast::channel(4096).0,
        sentry_guard: Arc::new(Mutex::new(None)),
    };

    // This isn't very secure, as the DSN is exposed within the binary
    let sentry_dsn = option_env!("SENTRY_DSN");

    let mut tauri_app = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init());

    if let Some(dsn) = sentry_dsn {
        debug!("Sentry enabled");
        let client = sentry::init((
            dsn,
            sentry::ClientOptions {
                release: sentry::release_name!(),
                debug: false,
                ..Default::default()
            },
        ));
        tauri_app = tauri_app.plugin(tauri_plugin_sentry::init(&client));
        *state.sentry_guard.lock().await = Some(client);
    } else {
        debug!("Sentry disabled");
    };

    let r = tauri_app
        .setup(|app| {
            app.manage(state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            app_version,
            host_platform,
            cmd::instance::del_instance,
            cmd::instance::new_instance,
            cmd::instance::start_instance,
            cmd::instance::stop_instance,
            cmd::misc::is_valid_url,
            cmd::runner::runner_info,
            cmd::runner::runner_list,
            cmd::runner::runner_new,
        ])
        .run(tauri::generate_context!());
    
        match r {
            Ok(_) => {}
            Err(e) => {
                error!(
                    "An error occurred while Volkanic Console was running: {}",
                    e
                );
                std::process::exit(1);
            }
        }

    match r {
        Ok(_) => {}
        Err(e) => {
            error!(
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
