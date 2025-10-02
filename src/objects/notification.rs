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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityMessageNotification {
    pub id: i32,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: NotificationType,
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<MessageActivity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiringNotification {
    pub id: i32,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<NotificationType>,
    #[serde(rename = "animeId")]
    pub anime_id: i32,
    pub episode: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<String>>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowingNotification {
    pub id: i32,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<NotificationType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationOption {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub notification_type: Option<NotificationType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedMediaAdditionNotification {
    pub id: i32,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<NotificationType>,
    #[serde(rename = "mediaId")]
    pub media_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadLikeNotification {
    pub id: i32,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<NotificationType>,
    #[serde(rename = "threadId")]
    pub thread_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<Thread>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityMentionNotification {
    pub id: i32,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: NotificationType,
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<ActivityUnion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityReplyNotification {
    pub id: i32,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: NotificationType,
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<ActivityUnion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityReplySubscribedNotification {
    pub id: i32,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: NotificationType,
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<ActivityUnion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityLikeNotification {
    pub id: i32,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: NotificationType,
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<ActivityUnion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityReplyLikeNotification {
    pub id: i32,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: NotificationType,
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<ActivityUnion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadCommentMentionNotification {
    pub id: i32,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<NotificationType>,
    #[serde(rename = "commentId")]
    pub comment_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<Thread>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<ThreadComment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadCommentReplyNotification {
    pub id: i32,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<NotificationType>,
    #[serde(rename = "commentId")]
    pub comment_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<Thread>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<ThreadComment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadCommentSubscribedNotification {
    pub id: i32,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<NotificationType>,
    #[serde(rename = "commentId")]
    pub comment_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<Thread>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<ThreadComment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadCommentLikeNotification {
    pub id: i32,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<NotificationType>,
    #[serde(rename = "commentId")]
    pub comment_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<Thread>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<ThreadComment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaDataChangeNotification {
    pub id: i32,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<NotificationType>,
    #[serde(rename = "mediaId")]
    pub media_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaMergeNotification {
    pub id: i32,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<NotificationType>,
    #[serde(rename = "mediaId")]
    pub media_id: i32,
    #[serde(rename = "deletedMediaTitles", skip_serializing_if = "Option::is_none")]
    pub deleted_media_titles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaDeletionNotification {
    pub id: i32,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<NotificationType>,
    #[serde(rename = "deletedMediaTitle", skip_serializing_if = "Option::is_none")]
    pub deleted_media_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
}

