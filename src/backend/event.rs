use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BackendEvent<T> {
    #[serde(rename = "payload")]
    pub payload: T,
}
