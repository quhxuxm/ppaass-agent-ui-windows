use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct AgentServerConfigUiBo {
    pub user_token: String,
    pub proxy_address: String,
    pub listening_port: String,
}
