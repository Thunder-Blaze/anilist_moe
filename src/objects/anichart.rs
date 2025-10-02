use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::objects::{common::Json, user::User};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniChartUser {
    pub user: Option<User>,
    pub settings: Option<Json>,
    pub highlights: Option<Json>,
}
