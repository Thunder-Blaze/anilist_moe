use serde::{Deserialize, Serialize};

use crate::{
    enums::recommendation::RecommendationRating,
    objects::{common::PageInfo, media::Media, user::User},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: i32,
    pub rating: Option<i32>,
    #[serde(rename = "userRating")]
    pub user_rating: Option<RecommendationRating>,
    pub media: Option<Media>,
    #[serde(rename = "mediaRecommendation")]
    pub media_recommendation: Option<Media>,
    pub context: Option<String>,
    #[serde(rename = "createdAt")]
    pub user: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationConnection {
    pub edges: Option<Vec<RecommendationEdge>>,
    pub nodes: Option<Vec<Recommendation>>,
    #[serde(rename = "pageInfo")]
    pub page_info: Option<PageInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationEdge {
    pub node: Option<Recommendation>,
}
