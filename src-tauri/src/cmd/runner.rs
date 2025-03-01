use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tauri::{
    AppHandle,
    Emitter,
    Manager
};
use tauri_plugin_dialog::DialogExt;
use tokio::sync::Mutex;
use tracing::info;
use uuid::Uuid;

use super::instance::UiInstance;

use crate::{
    runner::{Runner, RunnerConDetails, RunnerMode},
    AppState,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UiRunner {
    pub name: String,
    pub url: String,
    pub connected: bool,
    pub instances: HashMap<String, UiInstance>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RunnerInfoInterface {
    pub version: String,
    pub protocol: u64,
    pub mode: RunnerModeInterface,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RunnerModeInterface {
    #[serde(rename = "noAuth")]
    NoAuth,
}

#[tauri::command]
pub async fn runner_info(url: String) -> Result<RunnerInfoInterface, String> {
    let info = Runner::info(url).await.map_err(|e| e.to_string())?;

    Ok(RunnerInfoInterface {
        version: info.version,
        protocol: info.protocol,
        mode: match info.mode {
            RunnerMode::NoAuth => RunnerModeInterface::NoAuth,
        },
    })
}

#[tauri::command]
pub async fn runner_list(app: AppHandle) -> HashMap<String, UiRunner> {
    let state = app.state::<AppState>();

    to_ui_runners(state.runners.clone()).await
}

#[tauri::command]
pub async fn runner_new(app: AppHandle, name: String, url: String) -> Result<(), String> {
    let app = Arc::new(app);

    match m_runner_new(app.clone(), name, url).await {
        Ok(_) => {Ok(())},
        Err(e) => {
            app.dialog()
                .message(&e)
                .title("Runner Error")
                .show(|_| {});

            Err(e)
        }
    }
}

pub async fn send_runners(app: Arc<AppHandle>) -> Result<(), String> {
    let state = app.state::<AppState>();

    app.emit("runner", to_ui_runners(state.runners.clone()).await).map_err(|e| e.to_string())
}

async fn m_runner_new(app: Arc<AppHandle>, name: String, url: String) -> Result<(), String> {
    let state = app.state::<AppState>();

    let details = RunnerConDetails {
        name: name.clone(),
        url: url.trim_end_matches('/').to_string(),
    };

    let runner = Runner::new(details).await.map_err(|e| e.to_string())?;
    let uuid = Uuid::new_v4().to_string();

    if state.runners.lock().await.contains_key(&uuid) {
        return Err(String::from("Runner ID collision error"));
    };

    state.runners.lock().await.insert(uuid, runner.clone());

    info!("Added runner (\"{}\")", name);

    tokio::spawn(async move {
        loop {
            let _ = send_runners(app.clone()).await;

            runner.wait_for_update().await.unwrap();
        }
    });

    Ok(())
}

async fn to_ui_runners(runners: Arc<Mutex<HashMap<String, Arc<Runner>>>>) -> HashMap<String, UiRunner> {
    let mut ui_runners = HashMap::new();

    for r in runners.lock().await.iter() {
        let mut instances = HashMap::new();

        for i in r.1.get_instances().await {
            instances.insert(i.0, i.1.into());
        }

        let ui_r = UiRunner {
            name: r.1.get_name().await,
            url: r.1.get_url().await,
            connected: r.1.is_connected().await,
            instances,
        };

        ui_runners.insert(r.0.to_string(), ui_r.clone());
    }

    ui_runners
}
