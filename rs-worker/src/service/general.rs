use crate::model::general::{MessageResponse, PongResponse};

pub fn ping_response() -> PongResponse {
    PongResponse {
        status: "ok",
        message: "pong!",
    }
}

pub fn hello(name: String) -> MessageResponse {
    MessageResponse {
        status: "ok",
        message: format!("Hello, {name}!"),
    }
}

pub fn echo(msg: String) -> MessageResponse {
    MessageResponse {
        status: "ok",
        message: format!("echo says: {msg}"),
    }
}
