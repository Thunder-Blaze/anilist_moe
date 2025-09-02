use serde::{Deserialize, Serialize};

use crate::{enums::mods::ModActionType, objects::user::User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModAction {
    pub id: i32,
    pub user: Option<User>,
    pub mod_: Option<User>,
    #[serde(rename = "type")]
    pub action_type: Option<ModActionType>,
    #[serde(rename = "objectId")]
    pub object_id: Option<i32>,
    #[serde(rename = "objectType")]
    pub object_type: Option<String>,
    pub data: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
}
