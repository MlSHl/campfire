use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct PongResponse {
    pub status: &'static str,
    pub message: &'static str,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub status: &'static str,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Footstep {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub content: String,
    pub completed: i32,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ember {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub hours_today: f64,
    pub hours_this_week: f64,
    pub hours_total: f64,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub token_hash: String,
    pub created_at: String,
    pub expires_at: String,
}
