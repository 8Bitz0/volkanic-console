use reqwest::Client;
use std::time::Duration;

// Connection timeout in milliseconds
const DEFAULT_TIMEOUT_MS: u64 = 2000;

pub fn new_client() -> Result<Client, reqwest::Error> {
    Client::builder()
        .connect_timeout(Duration::from_millis(DEFAULT_TIMEOUT_MS))
        .user_agent(user_agent())
        .build()
}

fn user_agent() -> String {
    format!("volkanic-console/{}", env!("CARGO_PKG_VERSION"))
}
