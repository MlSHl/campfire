use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Footstep {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub content: String,
    pub completed: i32,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}
