use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct AgentConfigInfo {
    pub user_token: String,
    pub proxy_address: String,
    pub listening_port: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AgentServerSignalType {
    Info,
    Error,
    NetworkInfo {
        upload_bytes_amount: u32,
        upload_mb_per_second: f32,
        download_bytes_amount: u32,
        download_mb_per_second: f32,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentServerSignalPayload {
    #[serde(rename = "client_socket_address")]
    pub client_socket_address: Option<SocketAddr>,
    #[serde(rename = "message")]
    pub message: Option<String>,
    #[serde(rename = "signal_type")]
    pub signal_type: AgentServerSignalType,
}
