use serde::{Deserialize, Serialize};

use crate::{enums::review::ReviewRating, objects::{common::PageInfo, media::Media, user::User}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub id: i32,
    pub reporter: Option<User>,
    pub reported: Option<User>,
    pub reason: Option<String>,
    pub comments: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<i32>,
    pub cleared: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "mediaId")]
    pub media_id: i32,
    #[serde(rename = "mediaType")]
    pub media_type: Option<String>,
    pub summary: Option<String>,
    pub body: Option<String>,
    pub rating: Option<i32>,
    #[serde(rename = "ratingAmount")]
    pub rating_amount: Option<i32>,
    #[serde(rename = "userRating")]
    pub user_rating: Option<ReviewRating>,
    pub score: Option<i32>,
    pub private: Option<bool>,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    #[serde(rename = "updatedAt")]
    pub updated_at: i32,
    pub user: Option<User>,
    pub media: Option<Media>,
    pub spoilers: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewConnection {
    pub edges: Option<Vec<ReviewEdge>>,
    pub nodes: Option<Vec<Review>>,
    #[serde(rename = "pageInfo")]
    pub page_info: Option<PageInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewEdge {
    pub node: Option<Review>,
}