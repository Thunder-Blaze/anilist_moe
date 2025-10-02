use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    enums::review::ReviewRating,
    objects::{common::PageInfo, media::Media, user::User},
};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub id: i32,
    pub reporter: Option<User>,
    pub reported: Option<User>,
    pub reason: Option<String>,
    pub comments: Option<String>,
    pub created_at: Option<i32>,
    pub cleared: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Review {
    pub id: i32,
    pub user_id: Option<i32>,
    pub media_id: Option<i32>,
    pub media_type: Option<String>,
    pub summary: Option<String>,
    pub body: Option<String>,
    pub rating: Option<i32>,
    pub rating_amount: Option<i32>,
    pub user_rating: Option<ReviewRating>,
    pub score: Option<i32>,
    pub private: Option<bool>,
    pub site_url: Option<String>,
    pub created_at: Option<i32>,
    pub updated_at: Option<i32>,
    pub user: Option<User>,
    pub media: Option<Media>,
    pub spoilers: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewConnection {
    pub edges: Option<Vec<ReviewEdge>>,
    pub nodes: Option<Vec<Review>>,
    pub page_info: Option<PageInfo>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewEdge {
    pub node: Option<Review>,
}
