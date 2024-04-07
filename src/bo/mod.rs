pub mod payload;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FrontendArgument<T> {
    pub arg: T,
}

#[derive(Serialize, Deserialize)]
pub struct BackendEvent<T> {
    #[serde(rename = "payload")]
    pub payload: T,
}
