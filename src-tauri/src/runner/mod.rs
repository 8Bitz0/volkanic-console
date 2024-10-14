use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::broadcast;

mod http;

use http::new_client;

const SUPPORTED_PROTOCOL: u64 = 1;

pub struct RunnerClient {
    url: String,
    disconnect_tx: broadcast::Sender<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    GenericIo(std::io::Error),
    #[error("HTTP error: {0}")]
    HttpError(reqwest::Error),
    #[error("Response decode error")]
    ResponseDecode,
    #[error("Request encode error")]
    RequestEncode,
    #[error("Not a Volkanic Runner")]
    NotVolkanicRunner,
    #[error("Protocol version mismatch (found {0}, expected {1})")]
    /// Order: (local, remote)
    ProtocolMismatch(u64, u64)
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
    NoAuth
}

impl RunnerClient {
    pub async fn info<U: std::fmt::Display>(url: U) -> Result<RunnerInfo, Error> {
        let client = new_client().map_err(Error::HttpError)?;

        let info_raw = client
            .get(format!("{}/info", url))
            .send().await.map_err(Error::HttpError)?
            .text().await.map_err(Error::HttpError)?;
        let info_value = serde_json::from_str::<Value>(&info_raw)
            .map_err(|_| Error::ResponseDecode)?;

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
            eprintln!("Remote protocol mismatch (expected: {}, found: {})", SUPPORTED_PROTOCOL, protocol);
            return Err(Error::ProtocolMismatch(SUPPORTED_PROTOCOL, protocol));
        }

        let info = serde_json::from_str::<RunnerInfo>(&info_raw).map_err(|_| Error::ResponseDecode)?;

        Ok(info)
    }
    pub async fn new<U: std::fmt::Display>(url: U) -> Result<Self, Error> {
        let info = Self::info(&url).await?;

        Ok(Self {
            url: url.to_string(),
            disconnect_tx: broadcast::channel(1).0,
        })
    }
    pub async fn wait_for_close(&self) -> String {
        let mut rx = self.disconnect_tx.subscribe();

        match rx.recv().await {
            Ok(o) => o,
            Err(e) => format!("Receiver error: {}", e),
        }
    }
}
