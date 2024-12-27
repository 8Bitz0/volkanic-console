use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};
use tokio::{sync::{broadcast, Mutex}, time};
use tracing::{debug, info, error};

pub mod event;
pub mod instance;
mod http;

use instance::{Instance, InstanceRequest};

pub use http::is_valid_url;
use http::new_client;

const HEARTBEAT_INTERVAL_MS: u32 = 4000;
const HEARTBEAT_INTERVAL_OFFLINE_MS: u32 = 12000;
const SUPPORTED_PROTOCOL: u64 = 1;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RunnerConDetails {
    pub name: String,
    pub url: String,
}

pub struct Runner {
    details: Mutex<RunnerConDetails>,
    update: broadcast::Sender<()>,
    status_tx: broadcast::Sender<bool>,
    connected: Mutex<bool>,
    instances: Mutex<HashMap<String, Instance>>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(reqwest::Error),
    #[error("Response decode error")]
    ResponseDecode,
    #[error("Not a Volkanic Runner")]
    NotVolkanicRunner,
    #[error("Protocol version mismatch (found {0}, expected {1})")]
    /// Order: (local, remote)
    ProtocolMismatch(u64, u64),
    #[error("Broadcast receiver error")]
    BroadcastReceiver(broadcast::error::RecvError),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RunnerInfo {
    pub version: String,
    pub protocol: u64,
    pub mode: RunnerMode,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RunnerMode {
    #[serde(rename = "no-auth")]
    NoAuth,
}

impl Runner {
    pub async fn info<U: std::fmt::Display>(url: U) -> Result<RunnerInfo, Error> {
        let client = new_client().map_err(Error::Http)?;

        let info_raw = client
            .get(format!("{}/info", url))
            .send()
            .await
            .map_err(Error::Http)?
            .text()
            .await
            .map_err(Error::Http)?;
        let info_value =
            serde_json::from_str::<Value>(&info_raw).map_err(|_| Error::ResponseDecode)?;

        info!("Got remote info: {}", &info_raw);

        let protocol = match info_value["protocol"].as_u64() {
            Some(p) => p,
            None => {
                error!("Remote doesn't specify valid protocol");
                return Err(Error::NotVolkanicRunner);
            }
        };

        if protocol == SUPPORTED_PROTOCOL {
            info!("Remote protocol matched");
        } else {
            error!(
                "Remote protocol mismatch (expected: {}, found: {})",
                SUPPORTED_PROTOCOL, protocol
            );
            return Err(Error::ProtocolMismatch(SUPPORTED_PROTOCOL, protocol));
        }

        let info =
            serde_json::from_str::<RunnerInfo>(&info_raw).map_err(|_| Error::ResponseDecode)?;

        Ok(info)
    }
    pub async fn new(con_details: RunnerConDetails) -> Result<Arc<Self>, Error> {
        Self::info(&con_details.url).await?;

        let runner = Arc::new(Self {
            details: Mutex::new(con_details),
            // Only the sender is necessary since the receiver can be obtained
            // by calling the `subscribe()` method.
            update: broadcast::channel(255).0,
            status_tx: broadcast::channel(255).0,
            connected: Mutex::new(true),
            instances: Mutex::new(HashMap::new()),
        });

        Self::start_bg(runner.clone()).await;

        Ok(runner)
    }
    pub async fn wait_for_update(&self) -> Result<(), Error> {
        let mut rx = self.update.subscribe();

        rx.recv().await.map_err(Error::BroadcastReceiver)?;

        Ok(())
    }
    pub async fn wait_for_status(&self) -> bool {
        let mut rx = self.status_tx.subscribe();

        rx.recv().await.unwrap()
    }
    pub async fn get_name(&self) -> String {
        self.details.lock().await.name.to_string()
    }
    pub async fn get_url(&self) -> String {
        self.details.lock().await.url.to_string()
    }
    pub async fn is_connected(&self) -> bool {
        *self.connected.lock().await
    }
    /// Returns the local instances. Does not pull the latest data
    /// from the runner.
    pub async fn get_instances(&self) -> HashMap<String, Instance> {
        self.instances.lock().await.clone()
    }
    pub async fn update_all(&self) -> Result<(), Error> {
        debug!("Updating all instances");
        self.update_instances().await?;

        Ok(())
    }
    pub async fn update_instances(&self) -> Result<(), Error> {
        let client = new_client().map_err(Error::Http)?;

        debug!("Client requested, grabbing instances...");

        let instances_raw = client
            .get(format!("{}/instance/list", self.details.lock().await.url))
            .send()
            .await
            .map_err(Error::Http)?
            .text()
            .await
            .map_err(Error::Http)?;

        debug!("Got instance list: {}", &instances_raw);

        let instances = serde_json::from_str(&instances_raw).map_err(|_| Error::ResponseDecode)?;

        debug!("Decoded instance list");

        *self.instances.lock().await = instances;

        let _ = self.update.send(());

        Ok(())
    }
    pub async fn del_instance(&self, id: String) -> Result<(), Error> {
        let client = new_client().map_err(Error::Http)?;

        client
            .post(format!("{}/instance/{}/delete", self.details.lock().await.url, id))
            .send()
            .await
            .map_err(Error::Http)?;

        Ok(())
    }
    pub async fn new_instance(&self, instance: InstanceRequest) -> Result<(), Error> {
        let client = new_client().map_err(Error::Http)?;

        client
            .post(format!("{}/instance/new", self.details.lock().await.url))
            .json(&instance)
            .send()
            .await
            .map_err(Error::Http)?;

        Ok(())
    }
    pub async fn start_instance(&self, id: String) -> Result<(), Error> {
        let client = new_client().map_err(Error::Http)?;

        client
            .post(format!("{}/instance/{}/start", self.details.lock().await.url, id))
            .send()
            .await
            .map_err(Error::Http)?;

        Ok(())
    }
    pub async fn stop_instance(&self, id: String) -> Result<(), Error> {
        let client = new_client().map_err(Error::Http)?;

        client
            .post(format!("{}/instance/{}/stop", self.details.lock().await.url, id))
            .send()
            .await
            .map_err(Error::Http)?;

        Ok(())
    }
    fn send_update(&self) {
        let _ = self.update.send(());
    }
    fn send_status(&self, status: bool) {
        let _ = self.status_tx.send(status);
    }
    async fn heartbeat(&self) -> bool {
        let client = match new_client() {
            Ok(o) => o,
            Err(_) => return false,
        };

        let r = client
            .get(format!("{}/check", self.get_url().await))
            .send()
            .await;

        match r {
            Ok(r) => r.status() == StatusCode::OK,
            Err(_) => false,
        }
    }
    /// Starts background tasks for runner
    async fn start_bg(runner: Arc<Self>) {
        let runner = runner.clone();

        event::event_listen(runner.clone()).await;

        tokio::spawn(async move {
            loop {
                let connected = runner.heartbeat().await;

                let old_connected = *runner.connected.lock().await;
                *runner.connected.lock().await = connected;

                if connected != old_connected {
                    // Send update to all listeners
                    runner.send_update();
                    // Send new connection status to all listeners
                    runner.send_status(connected);
                }
    
                time::sleep(time::Duration::from_millis(
                    if connected {
                        HEARTBEAT_INTERVAL_MS as u64
                    } else {
                        HEARTBEAT_INTERVAL_OFFLINE_MS as u64
                    }
                )).await;
            }
        });
    }
}
