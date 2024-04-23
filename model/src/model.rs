use ppaass_protocol::message::values::address::PpaassUnifiedAddress;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub struct UiModelAgentServerConfiguration {
    pub user_token: String,
    pub proxy_address: Vec<String>,
    pub listening_port: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub enum UiModelStatusBarDetailType {
    Error,
    Warn,
    #[default]
    Info,
}

impl UiModelStatusBarDetailType {
    pub fn name(&self) -> &str {
        match self {
            UiModelStatusBarDetailType::Error => "error",
            UiModelStatusBarDetailType::Warn => "warn",
            UiModelStatusBarDetailType::Info => "info",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub struct UiModelStatusBarDetail {
    pub text: String,
    pub level: UiModelStatusBarDetailType,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct UiModelNetworkDetail {
    pub upload_mb_amount: f64,
    pub upload_mb_per_second: f64,
    pub download_mb_amount: f64,
    pub download_mb_per_second: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum UiModelBackendEvent {
    StartSuccess(u16),
    StartFail {
        listening_port: u16,
        reason: String,
    },
    StopSuccess,
    StopFail {
        listening_port: u16,
        reason: String,
    },
    Logging {
        client_socket_address: PpaassUnifiedAddress,
        src_address: Option<PpaassUnifiedAddress>,
        dst_address: Option<PpaassUnifiedAddress>,
        reason: Option<String>,
    },
    NetworkState {
        upload_mb_amount: f64,
        upload_mb_per_second: f64,
        download_mb_amount: f64,
        download_mb_per_second: f64,
    },
}
