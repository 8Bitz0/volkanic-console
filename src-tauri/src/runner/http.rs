use reqwest::Client;
use std::time::Duration;
use url::Url;

// Connection timeout in milliseconds
const DEFAULT_TIMEOUT_MS: u64 = 4000;

pub fn new_client() -> Result<Client, reqwest::Error> {
    Client::builder()
        .connect_timeout(Duration::from_millis(DEFAULT_TIMEOUT_MS))
        .user_agent(user_agent())
        .build()
}

pub fn is_valid_url<T: std::fmt::Display>(url: T) -> bool {
    Url::parse(&url.to_string()).is_ok()
}

fn user_agent() -> String {
    format!("volkanic-console/{}", env!("CARGO_PKG_VERSION"))
}
