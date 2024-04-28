use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AgentServerConfigurationVo {
    #[serde(rename = "userToken")]
    pub user_token: String,
    #[serde(rename = "proxyAddresses")]
    pub proxy_addresses: Vec<String>,
    #[serde(rename = "port")]
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AgentServerEventType {
    #[serde(rename = "NetworkState")]
    NetworkState,
    #[serde(rename = "StartSuccess")]
    StartSuccess,
    #[serde(rename = "StartFail")]
    StartFail,
    #[serde(rename = "StopSuccess")]
    StopSuccess,
    #[serde(rename = "StopFail")]
    StopFail,
    #[serde(rename = "LoggingError")]
    LoggingError,
    #[serde(rename = "LoggingInfo")]
    LoggingInfo,
    #[serde(rename = "LoggingWarn")]
    LoggingWarn,
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AgentServerEventVo {
    pub content: String,
    #[serde(rename = "eventType")]
    pub event_type: AgentServerEventType,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct NetworkStateVo {
    #[serde(rename = "uploadMbAmount")]
    pub upload_mb_amount: f64,
    #[serde(rename = "uploadMbPerSecond")]
    pub upload_mb_per_second: f64,
    #[serde(rename = "downloadMbAmount")]
    pub download_mb_amount: f64,
    #[serde(rename = "downloadMbPerSecond")]
    pub download_mb_per_second: f64,
}
