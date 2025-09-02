use serde::{Deserialize, Serialize};

use crate::objects::{common::Json, media::Media, user::User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thread {
    pub id: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "replyUserId")]
    pub reply_user_id: Option<i32>,
    #[serde(rename = "replyCommentId")]
    pub reply_comment_id: Option<i32>,
    #[serde(rename = "replyCount")]
    pub reply_count: Option<i32>,
    #[serde(rename = "viewCount")]
    pub view_count: Option<i32>,
    #[serde(rename = "isLocked")]
    pub is_locked: Option<bool>,
    #[serde(rename = "isSticky")]
    pub is_sticky: Option<bool>,
    #[serde(rename = "isSubscribed")]
    pub is_subscribed: Option<bool>,
    #[serde(rename = "likeCount")]
    pub like_count: i32,
    #[serde(rename = "isLiked")]
    pub is_liked: Option<bool>,
    #[serde(rename = "repliedAt")]
    pub replied_at: Option<i32>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    #[serde(rename = "updatedAt")]
    pub updated_at: i32,
    pub user: Option<User>,
    #[serde(rename = "replyUser")]
    pub reply_user: Option<User>,
    pub likes: Option<Vec<User>>,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
    pub categories: Option<Vec<ThreadCategory>>,
    #[serde(rename = "mediaCategories")]
    pub media_categories: Option<Vec<Media>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadCategory {
    pub id: i32,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadComment {
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
    #[serde(rename = "threadId")]
    pub thread_id: Option<i32>,
    pub comment: Option<String>,
    #[serde(rename = "asHtml")]
    pub as_html: Option<bool>,

    #[serde(rename = "likeCount")]
    pub like_count: i32,
    #[serde(rename = "isLiked")]
    pub is_liked: Option<bool>,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    #[serde(rename = "updatedAt")]
    pub updated_at: i32,
    pub thread: Option<Thread>,
    pub user: Option<User>,
    pub likes: Option<Vec<User>>,
    #[serde(rename = "childComments")]
    pub child_comments: Option<Json>,
    #[serde(rename = "isLocked")]
    pub is_locked: Option<bool>,
}
