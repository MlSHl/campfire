use serde::{Deserialize, Serialize};

use crate::model::general::Status;

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

impl From<Ember> for ClientEmber {
    fn from(value: Ember) -> Self {
        ClientEmber {
            id: value.id,
            name: value.name,
            hours_today: value.hours_today,
            hours_this_week: value.hours_this_week,
            hours_total: value.hours_total,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientEmber {
    pub id: String,
    pub name: String,
    pub hours_today: f64,
    pub hours_this_week: f64,
    pub hours_total: f64,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmberSyncRequest {
    pub embers: Vec<ClientEmber>,
    pub since: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmberSyncResponse {
    pub status: Status,
    pub embers: Vec<ClientEmber>,
    pub server_time: String,
}
