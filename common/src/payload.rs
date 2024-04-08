use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct AgentConfigInfo {
    pub user_token: String,
    pub proxy_address: String,
    pub listening_port: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AgentServerSignalLevel {
    Info,
    Error,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentServerSignalPayload {
    #[serde(rename = "client_socket_address")]
    pub client_socket_address: Option<SocketAddr>,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "level")]
    pub level: AgentServerSignalLevel,
}
