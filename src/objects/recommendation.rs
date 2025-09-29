use serde::{Deserialize, Serialize};

use crate::{
    enums::recommendation::RecommendationRating,
    objects::{common::PageInfo, media::Media, user::User},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<i32>,
    #[serde(rename = "userRating", skip_serializing_if = "Option::is_none")]
    pub user_rating: Option<RecommendationRating>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
    #[serde(rename = "mediaRecommendation", skip_serializing_if = "Option::is_none")]
    pub media_recommendation: Option<Media>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<RecommendationEdge>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Recommendation>>,
    #[serde(rename = "pageInfo", skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationEdge {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<Recommendation>,
}
