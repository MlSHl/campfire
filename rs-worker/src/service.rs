use crate::model::PongResponse;

pub fn get_ping() -> PongResponse {
    PongResponse {
        status: "ok",
        message: "pong!",
    }
}
