use serde::{Deserialize, Serialize};

use crate::{AppState, runner::{RunnerClient, RunnerMode}};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RunnerInfoInterface {
    pub version: String,
    pub protocol: u64,
    pub mode: RunnerModeInterface,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RunnerModeInterface {
    #[serde(rename = "noAuth")]
    NoAuth
}

#[tauri::command]
async fn runner_info(url: String) -> Result<RunnerInfoInterface, String> {
    let info = RunnerClient::info(url).await.map_err(|e| e.to_string())?;

    Ok(RunnerInfoInterface {
        version: info.version,
        protocol: info.protocol,
        mode: match info.mode {
            RunnerMode::NoAuth => RunnerModeInterface::NoAuth,
        },
    })
}
