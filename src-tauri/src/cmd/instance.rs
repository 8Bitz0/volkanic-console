use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;

use crate::{
    runner::instance::{
        Instance,
        InstanceRequest,
        InstanceType,
        VolkanicSource
    }, AppState
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UiInstance {
    name: String,
    inst_type: UiInstanceType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum UiInstanceType {
    Volkanic {
        source: UiVolkanicSource,
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum UiVolkanicSource {
    Url(String),
    Base64(String),
}

impl From<Instance> for UiInstance {
    fn from(value: Instance) -> Self {
        Self {
            name: value.name,
            inst_type: value.inst_type.into()
        }
    }
}

impl From<InstanceType> for UiInstanceType {
    fn from(value: InstanceType) -> Self {
        match value {
            InstanceType::Volkanic { source } => {
                UiInstanceType::Volkanic { source: source.into() }
            }
        }
    }
}

impl From<VolkanicSource> for UiVolkanicSource {
    fn from(value: VolkanicSource) -> Self {
        match value {
            VolkanicSource::Url(url) => {
                UiVolkanicSource::Url(url)
            }
            VolkanicSource::Base64(encoded) => {
                UiVolkanicSource::Base64(encoded)
            }
        }
    }
}

#[tauri::command]
pub async fn del_instance(app: AppHandle, runner: String, instance: String) -> Result<(), String> {
    let state = app.state::<AppState>();

    match state.runners.lock().await.get(&runner) {
        Some(runner) => {
            match runner.del_instance(instance).await {
                Ok(_) => {},
                Err(e) => {
                    app.dialog()
                        .message(e.to_string())
                        .title("Instance Error")
                        .show(|_| {});

                    return Err(e.to_string());
                }
            };
        }
        None => {
            app.dialog()
                .message("Runner not found")
                .title("Runner Error")
                .show(|_| {});

            return Err("Runner not found".to_string());
        }
    };

    Ok(())
}

#[tauri::command]
pub async fn new_instance(app: AppHandle, runner: String, instance: InstanceRequest) -> Result<(), String> {
    let state = app.state::<AppState>();

    match state.runners.lock().await.get(&runner) {
        Some(runner) => {
            match runner.new_instance(instance).await {
                Ok(_) => {},
                Err(e) => {
                    app.dialog()
                        .message(e.to_string())
                        .title("Instance Error")
                        .show(|_| {});

                    return Err(e.to_string());
                }
            };
        }
        None => {
            app.dialog()
                .message("Runner not found")
                .title("Runner Error")
                .show(|_| {});

            return Err("Runner not found".to_string());
        }
    };

    Ok(())
}
