use ppaass_protocol::message::values::address::PpaassUnifiedAddress;
use serde::Serialize;

pub static AGENT_SERVER_EVENT: &str = "__AGENT_SERVER_EVENT__";

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum AgentServerBackendToUiEvent {
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
