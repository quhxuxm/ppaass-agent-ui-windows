use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Error, Debug)]
pub enum PpaassAgentUiError {
    #[error("Agent error happen: {0}")]
    Agent(String),
    #[error("Other error happen: {0}")]
    Other(String),
}
