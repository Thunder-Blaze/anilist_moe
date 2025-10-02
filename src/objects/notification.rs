use crate::{
    enums::notification::NotificationType,
    objects::{
        activity::MessageActivity,
        media::Media,
        thread::{Thread, ThreadComment},
        user::User,
    },
    unions::activity::ActivityUnion,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityMessageNotification {
    pub id: i32,
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: Option<NotificationType>,
    pub activity_id: Option<i32>,
    pub context: Option<String>,
    pub created_at: Option<i32>,
    pub message: Option<MessageActivity>,
    pub user: Option<User>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiringNotification {
    pub id: i32,
    #[serde(rename = "type")]
    pub activity_type: Option<NotificationType>,
    pub anime_id: Option<i32>,
    pub episode: Option<i32>,
    pub contexts: Option<Vec<String>>,
    pub created_at: Option<i32>,
    pub media: Option<Media>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowingNotification {
    pub id: i32,
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: Option<NotificationType>,
    pub context: Option<String>,
    pub created_at: Option<i32>,
    pub user: Option<User>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationOption {
    #[serde(rename = "type")]
    pub notification_type: Option<NotificationType>,
    pub enabled: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedMediaAdditionNotification {
    pub id: i32,
    #[serde(rename = "type")]
    pub activity_type: Option<NotificationType>,
    pub media_id: Option<i32>,
    pub context: Option<String>,
    pub created_at: Option<i32>,
    pub media: Option<Media>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadLikeNotification {
    pub id: i32,
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: Option<NotificationType>,
    pub thread_id: Option<i32>,
    pub context: Option<String>,
    pub created_at: Option<i32>,
    pub thread: Option<Thread>,
    pub user: Option<User>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityMentionNotification {
    pub id: i32,
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: Option<NotificationType>,
    pub activity_id: Option<i32>,
    pub context: Option<String>,
    pub created_at: Option<i32>,
    pub activity: Option<ActivityUnion>,
    pub user: Option<User>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityReplyNotification {
    pub id: i32,
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: Option<NotificationType>,
    pub activity_id: Option<i32>,
    pub context: Option<String>,
    pub created_at: Option<i32>,
    pub activity: Option<ActivityUnion>,
    pub user: Option<User>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityReplySubscribedNotification {
    pub id: i32,
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: Option<NotificationType>,
    pub activity_id: Option<i32>,
    pub context: Option<String>,
    pub created_at: Option<i32>,
    pub activity: Option<ActivityUnion>,
    pub user: Option<User>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityLikeNotification {
    pub id: i32,
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: Option<NotificationType>,
    pub activity_id: Option<i32>,
    pub context: Option<String>,
    pub created_at: Option<i32>,
    pub activity: Option<ActivityUnion>,
    pub user: Option<User>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityReplyLikeNotification {
    pub id: i32,
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: Option<NotificationType>,
    pub activity_id: Option<i32>,
    pub context: Option<String>,
    pub created_at: Option<i32>,
    pub activity: Option<ActivityUnion>,
    pub user: Option<User>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadCommentMentionNotification {
    pub id: i32,
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: Option<NotificationType>,
    pub comment_id: Option<i32>,
    pub context: Option<String>,
    pub created_at: Option<i32>,
    pub thread: Option<Thread>,
    pub comment: Option<ThreadComment>,
    pub user: Option<User>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadCommentReplyNotification {
    pub id: i32,
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: Option<NotificationType>,
    pub comment_id: Option<i32>,
    pub context: Option<String>,
    pub created_at: Option<i32>,
    pub thread: Option<Thread>,
    pub comment: Option<ThreadComment>,
    pub user: Option<User>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadCommentSubscribedNotification {
    pub id: i32,
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: Option<NotificationType>,
    pub comment_id: Option<i32>,
    pub context: Option<String>,
    pub created_at: Option<i32>,
    pub thread: Option<Thread>,
    pub comment: Option<ThreadComment>,
    pub user: Option<User>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadCommentLikeNotification {
    pub id: i32,
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: Option<NotificationType>,
    pub comment_id: Option<i32>,
    pub context: Option<String>,
    pub created_at: Option<i32>,
    pub thread: Option<Thread>,
    pub comment: Option<ThreadComment>,
    pub user: Option<User>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaDataChangeNotification {
    pub id: i32,
    #[serde(rename = "type")]
    pub activity_type: Option<NotificationType>,
    pub media_id: Option<i32>,
    pub context: Option<String>,
    pub reason: Option<String>,
    pub created_at: Option<i32>,
    pub media: Option<Media>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaMergeNotification {
    pub id: i32,
    #[serde(rename = "type")]
    pub activity_type: Option<NotificationType>,
    pub media_id: Option<i32>,
    pub deleted_media_titles: Option<Vec<String>>,
    pub context: Option<String>,
    pub reason: Option<String>,
    pub created_at: Option<i32>,
    pub media: Option<Media>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaDeletionNotification {
    pub id: i32,
    #[serde(rename = "type")]
    pub activity_type: Option<NotificationType>,
    pub deleted_media_title: Option<String>,
    pub context: Option<String>,
    pub reason: Option<String>,
    pub created_at: Option<i32>,
}
