use serde::{Deserialize, Serialize};

use crate::runner::instance::{Instance, InstanceType, VolkanicSource};

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
