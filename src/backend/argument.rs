use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UiArgument<T> {
    pub arg: T
}
