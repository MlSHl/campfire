use crate::model::{Log, MessageResponse, PongResponse};
use crate::repository::logs::insert_log_for_user;
use worker::{console_log, Env};

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

pub async fn insert_log(env: &Env, log: Log) -> MessageResponse {
    let user_id = log.user_id.clone();
    let result = insert_log_for_user(env, log).await;
    match result {
        Ok(_) => MessageResponse {
            status: "ok",
            message: String::from("ok"),
        },
        Err(e) => {
            console_log!("{}", format!("Error inserting log for user {user_id}: {e}"));
            MessageResponse {
                status: "error",
                message: String::from("database error"),
            }
        }
    }
}
