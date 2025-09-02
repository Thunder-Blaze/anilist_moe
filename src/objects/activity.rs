use serde::{Deserialize, Serialize};
use crate::{
    enums::{activity::ActivityType},
    objects::{media::Media, user::User},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityReply {
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
    #[serde(rename = "activityId")]
    pub activity_id: Option<i32>,
    pub text: Option<String>,
    #[serde(rename = "likeCount")]
    pub like_count: i32,
    #[serde(rename = "isLiked")]
    pub is_liked: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    pub user: Option<User>,
    pub likes: Option<Vec<User>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextActivity {
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: ActivityType,
    #[serde(rename = "replyCount")]
    pub reply_count: i32,
    pub text: Option<String>,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
    #[serde(rename = "isLocked")]
    pub is_locked: Option<bool>,
    #[serde(rename = "isSubscribed")]
    pub is_subscribed: Option<bool>,
    #[serde(rename = "likeCount")]
    pub like_count: i32,
    #[serde(rename = "isLiked")]
    pub is_liked: Option<bool>,
    #[serde(rename = "isPinned")]
    pub is_pinned: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    pub user: Option<User>,
    pub replies: Option<Vec<ActivityReply>>,
    pub likes: Option<Vec<User>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListActivity {
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: ActivityType,
    #[serde(rename = "replyCount")]
    pub reply_count: i32,
    status: Option<String>,
    pub progress: Option<String>,
    #[serde(rename = "isLocked")]
    pub is_locked: Option<bool>,
    #[serde(rename = "isSubscribed")]
    pub is_subscribed: Option<bool>,
    #[serde(rename = "likeCount")]
    pub like_count: i32,
    #[serde(rename = "isLiked")]
    pub is_liked: Option<bool>,
    #[serde(rename = "isPinned")]
    pub is_pinned: Option<bool>,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    pub user: Option<User>,
    pub media: Option<Media>,
    pub replies: Option<Vec<ActivityReply>>,
    pub likes: Option<Vec<User>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageActivity {
    pub id: i32,
    pub recipient_id: Option<i32>,
    pub messenger_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: ActivityType,
    pub reply_count: i32,
    pub message: Option<String>,
    pub is_locked: Option<bool>,
    pub is_subscribed: Option<bool>,
    pub like_count: i32,
    pub is_liked: Option<bool>,
    pub is_private: Option<bool>,
    pub site_url: Option<String>,
    pub created_at: i32,
    pub recipient: Option<User>,
    pub messenger: Option<User>,
    pub replies: Option<Vec<ActivityReply>>,
    pub likes: Option<Vec<User>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListActivityOption {
    pub disabled: Option<bool>,
    #[serde(rename = "type")]
    pub activity_type: Option<ActivityType>,
}