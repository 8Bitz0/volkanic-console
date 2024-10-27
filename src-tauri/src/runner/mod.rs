use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::{sync::{broadcast, Mutex}, time};

mod http;

pub use http::is_valid_url;
use http::new_client;

const HEARTBEAT_INTERVAL_MS: u32 = 4000;
const SUPPORTED_PROTOCOL: u64 = 1;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RunnerConDetails {
    pub name: String,
    pub url: String,
}

pub struct Runner {
    details: Mutex<RunnerConDetails>,
    update: broadcast::Sender<()>,
    disconnect_tx: broadcast::Sender<String>,
    connected: Mutex<bool>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    GenericIo(std::io::Error),
    #[error("HTTP error: {0}")]
    HttpError(reqwest::Error),
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    #[error("Response decode error")]
    ResponseDecode,
    #[error("Request encode error")]
    RequestEncode,
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
        let client = new_client().map_err(Error::HttpError)?;

        let info_raw = client
            .get(format!("{}/info", url))
            .send()
            .await
            .map_err(Error::HttpError)?
            .text()
            .await
            .map_err(Error::HttpError)?;
        let info_value =
            serde_json::from_str::<Value>(&info_raw).map_err(|_| Error::ResponseDecode)?;

        println!("Got remote info: {}", &info_raw);

        let protocol = match info_value["protocol"].as_u64() {
            Some(p) => p,
            None => {
                eprintln!("Remote doesn't specify valid protocol");
                return Err(Error::NotVolkanicRunner);
            }
        };

        if protocol == SUPPORTED_PROTOCOL {
            println!("Remote protocol matched");
        } else {
            eprintln!(
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
        let info = Self::info(&con_details.url).await?;

        let runner = Arc::new(Self {
            details: Mutex::new(con_details),
            // Only the sender is necessary since the receiver can be obtained
            // by calling the `subscribe()` method.
            update: broadcast::channel(255).0,
            disconnect_tx: broadcast::channel(1).0,
            connected: Mutex::new(true),
        });

        Self::start_bg(runner.clone()).await;

        Ok(runner)
    }
    pub async fn wait_for_update(&self) -> Result<(), Error> {
        let mut rx = self.update.subscribe();

        rx.recv().await.map_err(Error::BroadcastReceiver)?;

        Ok(())
    }
    pub async fn wait_for_close(&self) -> String {
        let mut rx = self.disconnect_tx.subscribe();

        match rx.recv().await {
            Ok(o) => o,
            Err(e) => format!("Receiver error: {}", e),
        }
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

        tokio::spawn(async move {
            loop {
                let connected = runner.heartbeat().await;

                *runner.connected.lock().await = connected;

                let _ = runner.update.send(());
    
                time::sleep(time::Duration::from_millis(HEARTBEAT_INTERVAL_MS as u64)).await;
            }
        });
    }
}
