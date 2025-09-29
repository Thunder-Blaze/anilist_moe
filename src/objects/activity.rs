use crate::{
    enums::activity::ActivityType,
    objects::{media::Media, user::User},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityReply {
    pub id: i32,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "activityId", skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "likeCount")]
    pub like_count: i32,
    #[serde(rename = "isLiked", skip_serializing_if = "Option::is_none")]
    pub is_liked: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub likes: Option<Vec<User>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextActivity {
    pub id: i32,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: ActivityType,
    #[serde(rename = "replyCount")]
    pub reply_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "siteUrl", skip_serializing_if = "Option::is_none")]
    pub site_url: Option<String>,
    #[serde(rename = "isLocked", skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[serde(rename = "isSubscribed", skip_serializing_if = "Option::is_none")]
    pub is_subscribed: Option<bool>,
    #[serde(rename = "likeCount")]
    pub like_count: i32,
    #[serde(rename = "isLiked", skip_serializing_if = "Option::is_none")]
    pub is_liked: Option<bool>,
    #[serde(rename = "isPinned", skip_serializing_if = "Option::is_none")]
    pub is_pinned: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<ActivityReply>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub likes: Option<Vec<User>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListActivity {
    pub id: i32,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: ActivityType,
    #[serde(rename = "replyCount")]
    pub reply_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    #[serde(rename = "isLocked", skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[serde(rename = "isSubscribed", skip_serializing_if = "Option::is_none")]
    pub is_subscribed: Option<bool>,
    #[serde(rename = "likeCount")]
    pub like_count: i32,
    #[serde(rename = "isLiked", skip_serializing_if = "Option::is_none")]
    pub is_liked: Option<bool>,
    #[serde(rename = "isPinned", skip_serializing_if = "Option::is_none")]
    pub is_pinned: Option<bool>,
    #[serde(rename = "siteUrl", skip_serializing_if = "Option::is_none")]
    pub site_url: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<ActivityReply>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub likes: Option<Vec<User>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageActivity {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messenger_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: ActivityType,
    pub reply_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_subscribed: Option<bool>,
    pub like_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_liked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_url: Option<String>,
    pub created_at: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messenger: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<ActivityReply>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub likes: Option<Vec<User>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListActivityOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<ActivityType>,
}
