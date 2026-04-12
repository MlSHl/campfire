use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Ok,
    Error,
}

#[derive(Serialize)]
pub struct PongResponse {
    pub status: &'static str,
    pub message: &'static str,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub status: Status,
    pub message: String,
}

impl MessageResponse {
    pub fn new(status: Status, message: &str) -> Self {
        Self {
            status,
            message: message.to_string(),
        }
    }
}
