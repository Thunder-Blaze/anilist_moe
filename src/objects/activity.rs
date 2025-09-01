use serde::{Deserialize, Serialize};
use crate::{
    enums::activity::ActivityType,
    objects::user::User,
    objects::media::Media,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityReply {
    id: i32,
    user_id: Option<i32>,
    activity_id: Option<i32>,
    text: Option<String>,
    like_count: i32,
    is_liked: Option<bool>,
    created_at: i32,
    user: Option<User>,
    likes: Option<Vec<User>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextActivity {
    id: i32,
    #[serde(rename = "userId")]
    user_id: Option<i32>,
    #[serde(rename = "type")]
    activity_type: ActivityType,
    #[serde(rename = "replyCount")]
    reply_count: i32,
    text: Option<String>,
    #[serde(rename = "siteUrl")]
    site_url: Option<String>,
    #[serde(rename = "isLocked")]
    is_locked: Option<bool>,
    #[serde(rename = "isSubscribed")]
    is_subscribed: Option<bool>,
    #[serde(rename = "likeCount")]
    like_count: i32,
    #[serde(rename = "isLiked")]
    is_liked: Option<bool>,
    #[serde(rename = "isPinned")]
    is_pinned: Option<bool>,
    #[serde(rename = "createdAt")]
    created_at: i32,
    user: Option<User>,
    replies: Option<Vec<ActivityReply>>,
    likes: Option<Vec<User>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListActivity {
    id: i32,
    #[serde(rename = "userId")]
    user_id: Option<i32>,
    #[serde(rename = "type")]
    activity_type: ActivityType,
    #[serde(rename = "replyCount")]
    reply_count: i32,
    status: Option<String>,
    progress: Option<String>,
    #[serde(rename = "isLocked")]
    is_locked: Option<bool>,
    #[serde(rename = "isSubscribed")]
    is_subscribed: Option<bool>,
    #[serde(rename = "likeCount")]
    like_count: i32,
    #[serde(rename = "isLiked")]
    is_liked: Option<bool>,
    #[serde(rename = "isPinned")]
    is_pinned: Option<bool>,
    #[serde(rename = "siteUrl")]
    site_url: Option<String>,
    #[serde(rename = "createdAt")]
    created_at: i32,
    user: Option<User>,
    media: Option<Media>,
    replies: Option<Vec<ActivityReply>>,
    likes: Option<Vec<User>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageActivity {
    id: i32,
    recipient_id: Option<i32>,
    messenger_id: Option<i32>,
    #[serde(rename = "type")]
    activity_type: ActivityType,
    reply_count: i32,
    message: Option<String>,
    is_locked: Option<bool>,
    is_subscribed: Option<bool>,
    like_count: i32,
    is_liked: Option<bool>,
    is_private: Option<bool>,
    site_url: Option<String>,
    created_at: i32,
    recipient: Option<User>,
    messenger: Option<User>,
    replies: Option<Vec<ActivityReply>>,
    likes: Option<Vec<User>>,
}