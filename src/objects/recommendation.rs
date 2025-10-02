use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    enums::recommendation::RecommendationRating,
    objects::{common::PageInfo, media::Media, user::User},
};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recommendation {
    pub id: i32,
    pub rating: Option<i32>,
    pub user_rating: Option<RecommendationRating>,
    pub media: Option<Media>,
    pub media_recommendation: Option<Media>,
    pub context: Option<String>,
    pub user: Option<User>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationConnection {
    pub edges: Option<Vec<RecommendationEdge>>,
    pub nodes: Option<Vec<Recommendation>>,
    pub page_info: Option<PageInfo>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationEdge {
    pub node: Option<Recommendation>,
}
