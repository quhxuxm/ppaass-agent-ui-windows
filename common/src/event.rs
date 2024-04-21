use serde::Serialize;

pub static AGENT_SERVER_EVENT: &str = "__AGENT_SERVER_EVENT__";

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum AgentServerUiBackendEvent {
    AgentServerStartSuccess(u16),
    AgentServerStartFail(String),
    AgentServerStopSuccess,
    AgentServerStopFail(String),
    Logging(String),
    NetworkState {
        upload_bytes_amount: u64,
        upload_mb_per_second: f64,
        download_bytes_amount: u64,
        download_mb_per_second: f64,
    },
}
