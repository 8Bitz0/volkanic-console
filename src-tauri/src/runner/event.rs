use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, error};

use super::{
    http::new_client,
    instance::Instance,
    Runner
};

const RETRY_INTERVAL_MS: u64 = 1000;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RemoteEvent {
    #[serde(rename = "modify-instance")]
    ModifyInstance { id: String, instance: Instance },
    #[serde(rename = "delete-instance")]
    DeleteInstance { id: String },
}

pub async fn event_listen(runner: Arc<Runner>) {
    let url = format!("{}/events", runner.details.lock().await.url.clone());

    tokio::task::spawn(async move {
        loop {
            let url = url.clone();

            loop {
                debug!("Checking if connected");
                let connected = *runner.connected.lock().await;

                if connected {
                    debug!("Connected, starting event listener");
                    break;
                }

                debug!("Not connected, waiting for connection");

                runner.wait_for_status().await;
            }

            let client = match new_client() {
                Ok(o) => o,
                Err(e) => {
                    error!("Got error while creating client for SSE listener: {}", e);
                    break;
                },
            };

            debug!("Sending SSE request");

            let r = match client
                .get(url)
                .send().await {
                Ok(o) => o,
                Err(e) => {
                    error!("Got error requesting SSE: {}", e);
                    wait().await;
                    continue;
                }
            };

            debug!("SSE request succeeded");
            debug!("Refreshing everything");

            match runner.update_all().await {
                Ok(_) => {},
                Err(e) => {
                    error!("An error occurred while updating all instances: {}", e);
                    wait().await;
                    continue;
                }
            };
            
            let mut stream = r.bytes_stream();

            loop {
                tokio::select! {
                    _ = runner.wait_for_status() => {
                        // This forces a full refresh and reconnection
                        break;
                    }
                    event_raw = stream.next() => {
                        let event_raw = match event_raw {
                            Some(Ok(o)) => o,
                            Some(Err(e)) => {
                                error!("An error occurred while receiving event: {e}");
                                break;
                            },
                            None => break,
                        };

                        debug!("Received remote event");

                        // Decode the raw event bytes into a UTF8 string
                        let mut event_str = match std::str::from_utf8(&event_raw) {
                            Ok(o) => o,
                            Err(e) => {
                                error!("An error occurred while decoding UTF8 in event: {e}");
                                break;
                            }
                        };

                        debug!("Got remote raw event: {}", event_str);

                        // Check for the "data: " prefix
                        if event_str.starts_with("data: ") {
                            debug!("Stripped prefix from event string, raw: {}", event_str);
                            // Strips the prefix from the raw event string
                            event_str = &event_str[6..];
                        } else {
                            error!("Received non-data in event listener");
                            break;
                        }

                        let event = match serde_json::from_str::<RemoteEvent>(event_str) {
                            Ok(o) => o,
                            Err(e) => {
                                error!("An error occurred while parsing remote event: {e}");
                                break;
                            }
                        };

                        match event {
                            RemoteEvent::ModifyInstance { id, instance } => {
                                debug!("Got remote modify event (ID: {}, Instance: {:?})", id, instance);
                                runner.instances.lock().await.insert(id, instance);
                                runner.send_update();
                            }
                            RemoteEvent::DeleteInstance { id } => {
                                debug!("Got remote delete event (ID: {})", id);
                                runner.instances.lock().await.remove(&id);
                                runner.send_update();
                            }
                        }
                    }
                }
            }
        }
    });
}

async fn wait() {
    tokio::time::sleep(tokio::time::Duration::from_millis(RETRY_INTERVAL_MS)).await;
}
