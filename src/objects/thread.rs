use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::objects::{common::Json, media::Media, user::User};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thread {
    pub id: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub user_id: Option<i32>,
    pub reply_user_id: Option<i32>,
    pub reply_comment_id: Option<i32>,
    pub reply_count: Option<i32>,
    pub view_count: Option<i32>,
    pub is_locked: Option<bool>,
    pub is_sticky: Option<bool>,
    pub is_subscribed: Option<bool>,
    pub like_count: Option<i32>,
    pub is_liked: Option<bool>,
    pub replied_at: Option<i32>,
    pub created_at: Option<i32>,
    pub updated_at: Option<i32>,
    pub user: Option<User>,
    pub reply_user: Option<User>,
    pub likes: Option<Vec<User>>,
    pub site_url: Option<String>,
    pub categories: Option<Vec<ThreadCategory>>,
    pub media_categories: Option<Vec<Media>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadCategory {
    pub id: i32,
    pub name: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadComment {
    pub id: i32,
    pub user_id: Option<i32>,
    pub thread_id: Option<i32>,
    pub comment: Option<String>,
    pub as_html: Option<bool>,

    pub like_count: Option<i32>,
    pub is_liked: Option<bool>,
    pub site_url: Option<String>,
    pub created_at: Option<i32>,
    pub updated_at: Option<i32>,
    pub thread: Option<Thread>,
    pub user: Option<User>,
    pub likes: Option<Vec<User>>,
    pub child_comments: Option<Json>,
    pub is_locked: Option<bool>,
}
