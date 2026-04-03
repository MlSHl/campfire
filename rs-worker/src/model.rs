use serde::Serialize;

#[derive(Serialize)]
pub struct PongResponse {
    pub status: &'static str,
    pub message: &'static str,
}
