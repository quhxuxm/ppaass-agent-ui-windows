use ppaass_protocol::message::values::address::PpaassUnifiedAddress;
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
pub enum AgentServerEventVo {
    NetworkState {
        #[serde(rename = "uploadMbAmount")]
        upload_mb_amount: f64,
        #[serde(rename = "uploadMbPerSecond")]
        upload_mb_per_second: f64,
        #[serde(rename = "downloadMbAmount")]
        download_mb_amount: f64,
        #[serde(rename = "downloadMbPerSecond")]
        download_mb_per_second: f64,
    },
    StartSuccess(#[serde(rename = "port")] u16),
    StartFail {
        port: u16,
        reason: String,
    },
    StopSuccess,
    StopFail {
        port: u16,
        reason: String,
    },
    Logging {
        #[serde(rename = "clientSocketAddress")]
        client_socket_address: PpaassUnifiedAddress,
        #[serde(rename = "srcAddress")]
        src_address: Option<PpaassUnifiedAddress>,
        #[serde(rename = "dstAddress")]
        dst_address: Option<PpaassUnifiedAddress>,
        reason: Option<String>,
    },
}
