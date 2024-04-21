use std::num::ParseIntError;

use ppaass_agent::error::AgentServerError;
use tauri::{Error as TauriError, InvokeError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AgentServerUiError {
    #[error(transparent)]
    Backend(#[from] AgentServerError),
    #[error(transparent)]
    Tauri(TauriError),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

impl From<AgentServerUiError> for InvokeError {
    fn from(value: AgentServerUiError) -> Self {
        InvokeError::from(format!("{value:?}"))
    }
}
