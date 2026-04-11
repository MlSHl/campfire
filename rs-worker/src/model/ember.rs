use serde::{Deserialize, Serialize};

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

