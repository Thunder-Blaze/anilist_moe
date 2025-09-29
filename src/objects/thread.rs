use serde::{Deserialize, Serialize};

use crate::objects::{common::Json, media::Media, user::User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thread {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "replyUserId", skip_serializing_if = "Option::is_none")]
    pub reply_user_id: Option<i32>,
    #[serde(rename = "replyCommentId", skip_serializing_if = "Option::is_none")]
    pub reply_comment_id: Option<i32>,
    #[serde(rename = "replyCount", skip_serializing_if = "Option::is_none")]
    pub reply_count: Option<i32>,
    #[serde(rename = "viewCount", skip_serializing_if = "Option::is_none")]
    pub view_count: Option<i32>,
    #[serde(rename = "isLocked", skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[serde(rename = "isSticky", skip_serializing_if = "Option::is_none")]
    pub is_sticky: Option<bool>,
    #[serde(rename = "isSubscribed", skip_serializing_if = "Option::is_none")]
    pub is_subscribed: Option<bool>,
    #[serde(rename = "likeCount")]
    pub like_count: i32,
    #[serde(rename = "isLiked", skip_serializing_if = "Option::is_none")]
    pub is_liked: Option<bool>,
    #[serde(rename = "repliedAt", skip_serializing_if = "Option::is_none")]
    pub replied_at: Option<i32>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    #[serde(rename = "updatedAt")]
    pub updated_at: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(rename = "replyUser", skip_serializing_if = "Option::is_none")]
    pub reply_user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub likes: Option<Vec<User>>,
    #[serde(rename = "siteUrl", skip_serializing_if = "Option::is_none")]
    pub site_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<ThreadCategory>>,
    #[serde(rename = "mediaCategories", skip_serializing_if = "Option::is_none")]
    pub media_categories: Option<Vec<Media>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadCategory {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadComment {
    pub id: i32,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "threadId", skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "asHtml", skip_serializing_if = "Option::is_none")]
    pub as_html: Option<bool>,

    #[serde(rename = "likeCount")]
    pub like_count: i32,
    #[serde(rename = "isLiked", skip_serializing_if = "Option::is_none")]
    pub is_liked: Option<bool>,
    #[serde(rename = "siteUrl", skip_serializing_if = "Option::is_none")]
    pub site_url: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    #[serde(rename = "updatedAt")]
    pub updated_at: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<Thread>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub likes: Option<Vec<User>>,
    #[serde(rename = "childComments", skip_serializing_if = "Option::is_none")]
    pub child_comments: Option<Json>,
    #[serde(rename = "isLocked", skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
}
