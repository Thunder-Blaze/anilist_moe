use serde::{Deserialize, Serialize};

use crate::{
    enums::review::ReviewRating,
    objects::{common::PageInfo, media::Media, user::User},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporter: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleared: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "mediaId")]
    pub media_id: i32,
    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<i32>,
    #[serde(rename = "ratingAmount", skip_serializing_if = "Option::is_none")]
    pub rating_amount: Option<i32>,
    #[serde(rename = "userRating", skip_serializing_if = "Option::is_none")]
    pub user_rating: Option<ReviewRating>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(rename = "siteUrl", skip_serializing_if = "Option::is_none")]
    pub site_url: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    #[serde(rename = "updatedAt")]
    pub updated_at: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoilers: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<ReviewEdge>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Review>>,
    #[serde(rename = "pageInfo", skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewEdge {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<Review>,
}
