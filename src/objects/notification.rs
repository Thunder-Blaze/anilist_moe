use serde::{Deserialize, Serialize};
use crate::{
    enums::notification::NotificationType,
    objects::{activity::MessageActivity, media::Media, user::User}, unions::activity::ActivityUnion,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityNotification {
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "type")]
    pub activity_type: NotificationType,
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub context: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    pub activity: Option<ActivityUnion>,
    pub user: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityMessageNotification {
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "type")]
    pub activity_type: NotificationType,
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub context: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    pub message: Option<MessageActivity>,
    pub user: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiringNotification {
    pub id: i32,
    #[serde(rename = "type")]
    pub activity_type: Option<NotificationType>,
    #[serde(rename = "animeId")]
    pub anime_id: i32,
    pub episode: i32,
    pub contexts: Option<Vec<String>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<i32>,
    pub media: Option<Media>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationOption {
    #[serde(rename = "type")]
    pub notification_type: Option<NotificationType>,
    pub enabled: Option<bool>,
}