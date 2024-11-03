use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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

            let client = match new_client() {
                Ok(o) => o,
                Err(e) => {
                    println!("Got error while creating client for SSE listener: {}", e);

                    break;
                },
            };

            let r = match client
                .get(url)
                .send().await {
                Ok(o) => o,
                Err(e) => {
                    eprintln!("Got error requesting SSE: {}", e);

                    wait().await;

                    continue;
                }
            };
            
            let mut stream = r.bytes_stream();
            while let Some(event_raw) = stream.next().await {
                let event_raw = match event_raw {
                    Ok(o) => o,
                    Err(e) => {
                        eprintln!("An error occurred while receiving event: {e}");

                        continue;
                    },
                };

                let mut event_str = match std::str::from_utf8(&event_raw) {
                    Ok(o) => o,
                    Err(e) => {
                        eprintln!("An error occurred while decoding UTF8 in event: {e}");

                        continue;
                    }
                };

                if event_str.starts_with("data: ") {
                    event_str = &event_str[6..];
                } else {
                    println!("Received not-data in event listener");

                    continue;
                }

                let event = match serde_json::from_str::<RemoteEvent>(event_str) {
                    Ok(o) => {
                        o
                    }
                    Err(e) => {
                        eprintln!("An error occurred while parsing remote event: {e}");

                        continue;
                    }
                };

                match event {
                    RemoteEvent::ModifyInstance { id, instance } => {
                        runner.instances.lock().await.insert(id, instance);

                        runner.send_update();
                    }
                    RemoteEvent::DeleteInstance { id } => {
                        runner.instances.lock().await.remove(&id);

                        runner.send_update();
                    }
                }
            }
        }
    });
}

async fn wait() {
    tokio::time::sleep(tokio::time::Duration::from_millis(RETRY_INTERVAL_MS)).await;
}
