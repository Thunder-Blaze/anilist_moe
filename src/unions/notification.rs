use crate::objects::notification::{
    ActivityNotification, ActivityMessageNotification, AiringNotification,
    FollowingNotification, RelatedMediaAdditionNotification,
    ThreadCommentNotification, ThreadLikeNotification
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NotificationUnion {
    #[serde(rename = "AIRING")]
    Airing(AiringNotification),
    #[serde(rename = "FOLLOWING")]
    Following(FollowingNotification),
    #[serde(rename = "ACTIVITY_MESSAGE")]
    ActivityMessage(ActivityMessageNotification),
    #[serde(rename = "ACTIVITY_REPLY")]
    ActivityReply(ActivityNotification),
    #[serde(rename = "ACTIVITY_MENTION")]
    ActivityMention(ActivityNotification),
    #[serde(rename = "ACTIVITY_LIKE")]
    ActivityLike(ActivityNotification),
    #[serde(rename = "THREAD_COMMENT_MESSAGE")]
    ThreadCommentMessage(ThreadCommentNotification),
    #[serde(rename = "THREAD_SUBSCRIBED")]
    ThreadSubscribed(ThreadCommentNotification),
    #[serde(rename = "THREAD_COMMENT_REPLY")]
    ThreadCommentReply(ThreadCommentNotification),
    #[serde(rename = "THREAD_LIKE")]
    ThreadLike(ThreadLikeNotification),
    #[serde(rename = "RELATED_MEDIA_ADDITION")]
    RelatedMediaAddition(RelatedMediaAdditionNotification),
}
