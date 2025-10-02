use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{enums::mods::ModActionType, objects::user::User};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModAction {
    pub id: i32,
    pub user: Option<User>,
    pub mod_: Option<User>,
    #[serde(rename = "type")]
    pub action_type: Option<ModActionType>,
    pub object_id: Option<i32>,
    pub object_type: Option<String>,
    pub data: Option<String>,
    pub created_at: Option<i32>,
}
