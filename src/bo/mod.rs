pub mod command;
pub mod ui_state;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UiBackendCommandArgWrapper<T> {
    pub arg: T,
}

#[derive(Serialize, Deserialize)]
pub struct UiBackendEventWrapper<T> {
    #[serde(rename = "payload")]
    pub payload: T,
}
