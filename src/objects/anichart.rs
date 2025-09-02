use serde::{Deserialize, Serialize};

use crate::objects::{common::Json, user::User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AniChartUser {
    pub user: Option<User>,
    pub settings: Option<Json>,
    pub highlights: Option<Json>,
}