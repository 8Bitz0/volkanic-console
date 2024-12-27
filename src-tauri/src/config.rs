use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};
use tokio::{fs, io::AsyncWriteExt};

use crate::runner::RunnerConDetails;

const CONF_FILE_NAME: &str = "config.json";
const DIR_NAME: &str = "Volkanic Console";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(std::io::Error),
    #[error("Expected file, found directory: {0}")]
    FoundDirectory(PathBuf),
    #[error("Expected directory, found file: {0}")]
    FoundFile(PathBuf),
    #[error("JSON decode error: {0}")]
    JsonDecode(serde_jsonc::Error),
    #[error("JSON encode error: {0}")]
    JsonEncode(serde_jsonc::Error),
    #[error("Cannot determine config directory")]
    NoConfigDir,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Config {
    pub runners: HashMap<String, RunnerConDetails>,
}

pub struct ConfigFile {
    pub config: Config,
    path: PathBuf,
}

impl ConfigFile {
    pub async fn new() -> Result<Self, Error> {
        let dir = match dirs::config_dir() {
            Some(o) => o.join(DIR_NAME),
            None => return Err(Error::NoConfigDir),
        };

        if dir.is_file() {
            return Err(Error::FoundFile(dir));
        } else if !dir.is_dir() {
            fs::create_dir_all(&dir).await.map_err(Error::Io)?;
        }

        let path = dir.join(CONF_FILE_NAME);

        if path.is_file() {
            let config_raw = fs::read_to_string(&path).await.map_err(Error::Io)?;

            let config = serde_jsonc::from_str(&config_raw).map_err(Error::JsonDecode)?;

            Ok(ConfigFile { path, config })
        } else if path.is_dir() {
            Err(Error::FoundDirectory(path))
        } else {
            let config_file = Self {
                path,
                config: Config::default(),
            };

            config_file.update().await?;

            Ok(config_file)
        }
    }
    pub async fn update(&self) -> Result<(), Error> {
        let mut f = fs::File::create(&self.path).await.map_err(Error::Io)?;

        let mut config_raw =
            serde_jsonc::to_string_pretty(&self.config).map_err(Error::JsonEncode)?;
        config_raw.push('\n');

        f.write_all(config_raw.as_bytes())
            .await
            .map_err(Error::Io)?;

        Ok(())
    }
}
