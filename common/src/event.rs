use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentEvent {
    #[display(fmt = "__BACKEND_AGENT_SERVER_START__")]
    Start,
    #[display(fmt = "__BACKEND_AGENT_SERVER_STOP__")]
    Stop,
    #[display(fmt = "__BACKEND_AGENT_SERVER_SIGNAL__")]
    Signal,
}
