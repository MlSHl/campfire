use worker::{console_log, Env};

use crate::model::ember::{EmberSyncRequest, EmberSyncResponse};
use crate::model::general::Status;

pub async fn sync(env: &Env, ember_sync_req: EmberSyncRequest) -> EmberSyncResponse {
    EmberSyncResponse {
        status: Status::Ok,
        embers: vec![],
        server_time: "temp".to_string(),
    }
}
