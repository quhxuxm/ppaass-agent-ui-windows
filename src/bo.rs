use derive_more::Display;
use serde::{Deserialize, Serialize};
#[derive(Debug, Display)]
pub enum UiBackendCommand {
    #[display(fmt = "start_agent_server")]
    StartAgentServer,
    #[display(fmt = "stop_agent_server")]
    StopAgentServer,
    #[display(fmt = "load_agent_server_configuration")]
    LoadAgentServerConifugration,
}

#[derive(Serialize, Deserialize)]
pub struct UiBackendCommandArgWrapper<T> {
    pub arg: T,
}

#[derive(Serialize, Deserialize)]
pub struct UiBackendEventWrapper<T> {
    #[serde(rename = "payload")]
    pub payload: T,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Display)]
pub enum UiStatusBarMessageLevel {
    #[display(fmt = "error")]
    Error,
    #[display(fmt = "info")]
    #[default]
    Info,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct UiStatusBarDetail {
    pub text: String,
    pub level: UiStatusBarMessageLevel,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct UiNetworkDetail {
    pub upload_mb_amount: u64,
    pub upload_mb_per_second: f64,
    pub download_mb_amount: u64,
    pub download_mb_per_second: f64,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct UiConfigurationForm {
    pub user_token: String,
    pub proxy_address: String,
    pub listening_port: String,
}
