use crate::{
    enums::activity::ActivityType,
    objects::{media::Media, user::User},
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityReply {
    pub id: i32,
    pub user_id: Option<i32>,
    pub activity_id: Option<i32>,
    pub text: Option<String>,
    pub like_count: Option<i32>,
    pub is_liked: Option<bool>,
    pub created_at: Option<i32>,
    pub user: Option<User>,
    pub likes: Option<Vec<User>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextActivity {
    pub id: i32,
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: Option<ActivityType>,
    pub reply_count: Option<i32>,
    pub text: Option<String>,
    pub site_url: Option<String>,
    pub is_locked: Option<bool>,
    pub is_subscribed: Option<bool>,
    pub like_count: Option<i32>,
    pub is_liked: Option<bool>,
    pub is_pinned: Option<bool>,
    pub created_at: Option<i32>,
    pub user: Option<User>,
    pub replies: Option<Vec<ActivityReply>>,
    pub likes: Option<Vec<User>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListActivity {
    pub id: i32,
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: Option<ActivityType>,
    pub reply_count: Option<i32>,
    pub status: Option<String>,
    pub progress: Option<String>,
    pub is_locked: Option<bool>,
    pub is_subscribed: Option<bool>,
    pub like_count: Option<i32>,
    pub is_liked: Option<bool>,
    pub is_pinned: Option<bool>,
    pub site_url: Option<String>,
    pub created_at: Option<i32>,
    pub user: Option<User>,
    pub media: Option<Media>,
    pub replies: Option<Vec<ActivityReply>>,
    pub likes: Option<Vec<User>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageActivity {
    pub id: i32,
    pub recipient_id: Option<i32>,
    pub messenger_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: Option<ActivityType>,
    pub reply_count: Option<i32>,
    pub message: Option<String>,
    pub is_locked: Option<bool>,
    pub is_subscribed: Option<bool>,
    pub like_count: Option<i32>,
    pub is_liked: Option<bool>,
    pub is_private: Option<bool>,
    pub site_url: Option<String>,
    pub created_at: Option<i32>,
    pub recipient: Option<User>,
    pub messenger: Option<User>,
    pub replies: Option<Vec<ActivityReply>>,
    pub likes: Option<Vec<User>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListActivityOption {
    pub disabled: Option<bool>,
    pub activity_type: Option<ActivityType>,
}
