pub mod command;
pub mod ui_state;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BackendCommandArgumentWrapper<T> {
    pub arg: T,
}

#[derive(Serialize, Deserialize)]
pub struct BackendEventWrapper<T> {
    #[serde(rename = "payload")]
    pub payload: T,
}
