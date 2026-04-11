use crate::model::general::MessageResponse;
use crate::model::logbook::Log;

use crate::repository::logs::insert_log_for_user;

use worker::{console_error, Env};

pub async fn insert_log(env: &Env, log: Log) -> MessageResponse {
    let user_id = log.user_id.clone();
    let result = insert_log_for_user(env, log).await;
    match result {
        Ok(_) => MessageResponse {
            status: "ok",
            message: String::from("ok"),
        },
        Err(e) => {
            console_error!("{}", format!("Error inserting log for user {user_id}: {e}"));
            MessageResponse {
                status: "error",
                message: String::from("database error"),
            }
        }
    }
}
