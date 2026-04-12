use crate::model::general::{MessageResponse, PongResponse, Status};

pub fn ping_response() -> PongResponse {
    PongResponse {
        status: "ok",
        message: "pong!",
    }
}

pub fn hello(name: String) -> MessageResponse {
    let message = format!("Hello {name}");
    MessageResponse::new(Status::Ok, &message)
}

pub fn echo(msg: String) -> MessageResponse {
    let message = format!("echo says: {msg}");
    MessageResponse::new(Status::Ok, &message)
}
