use serde::Serialize;

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
