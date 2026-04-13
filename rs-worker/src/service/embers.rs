use std::collections::HashMap;

use worker::Env;

use crate::model::ember::{Ember, EmberSyncRequest, EmberSyncResponse};
use crate::model::general::Status;
use crate::repository;

pub async fn sync(env: &Env, user_id: &str, ember_sync_req: EmberSyncRequest) -> EmberSyncResponse {
    let remote_embers = match repository::embers::get_all_by_user(env, user_id).await {
        Ok(embers) => embers,
        Err(_) => {
            return EmberSyncResponse {
                status: Status::Error,
                embers: vec![],
                server_time: chrono::Utc::now().to_rfc3339(),
            };
        }
    };

    let remote_map: std::collections::HashMap<String, Ember> = remote_embers
        .into_iter()
        .map(|ember| (ember.id.clone(), ember))
        .collect();

    for client_ember in &ember_sync_req.embers {
        match remote_map.get(&client_ember.id) {
            None => {
                if repository::embers::insert(env, user_id, client_ember)
                    .await
                    .is_err()
                {
                    return EmberSyncResponse {
                        status: Status::Error,
                        embers: vec![],
                        server_time: chrono::Utc::now().to_rfc3339(),
                    };
                }
            }
            Some(remote_ember) => {
                if client_ember.updated_at > remote_ember.updated_at {
                    if repository::embers::update(env, user_id, client_ember)
                        .await
                        .is_err()
                    {
                        return EmberSyncResponse {
                            status: Status::Error,
                            embers: vec![],
                            server_time: chrono::Utc::now().to_rfc3339(),
                        };
                    }
                }
            }
        }
    }

    let final_embers = match repository::embers::get_all_by_user(env, user_id).await {
        Ok(embers) => embers,
        Err(_) => {
            return EmberSyncResponse {
                status: Status::Error,
                embers: vec![],
                server_time: chrono::Utc::now().to_rfc3339(),
            };
        }
    };

    EmberSyncResponse {
        status: Status::Ok,
        embers: final_embers.into_iter().map(Into::into).collect(),
        server_time: chrono::Utc::now().to_rfc3339(),
    }
}
