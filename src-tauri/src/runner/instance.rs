use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Instance {
    pub name: String,
    #[serde(rename = "type")]
    pub inst_type: InstanceType,
    pub status: InstanceStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum InstanceType {
    #[serde(rename = "volkanic")]
    Volkanic { source: VolkanicSource },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum InstanceStatus {
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "stopping")]
    Stopping,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum VolkanicSource {
    #[serde(rename = "url")]
    Url(String),
    #[serde(rename = "base64")]
    Base64(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub inst_type: InstanceType,
}
