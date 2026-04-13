use crate::model::general::Status;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub token_hash: String,
    pub created_at: String,
    pub expires_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordHashRow {
    pub password_hash: String,
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    pub status: Status,
    pub message: String,
    pub raw_token: Option<String>,
}

impl LoginResponse {
    pub fn new(status: Status, message: &str, raw_token: Option<String>) -> Self {
        Self {
            status,
            message: message.to_string(),
            raw_token,
        }
    }
}
