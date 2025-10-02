use crate::objects::notification::{
    ActivityMessageNotification, ActivityMentionNotification,
    ActivityReplyNotification, ActivityReplySubscribedNotification, ActivityLikeNotification,
    ActivityReplyLikeNotification, AiringNotification, FollowingNotification,
    RelatedMediaAdditionNotification, ThreadCommentMentionNotification,
    ThreadCommentReplyNotification, ThreadCommentSubscribedNotification,
    ThreadCommentLikeNotification, ThreadLikeNotification, MediaDataChangeNotification,
    MediaMergeNotification, MediaDeletionNotification
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
    #[serde(rename = "ACTIVITY_MENTION")]
    ActivityMention(ActivityMentionNotification),
    #[serde(rename = "ACTIVITY_REPLY")]
    ActivityReply(ActivityReplyNotification),
    #[serde(rename = "ACTIVITY_REPLY_SUBSCRIBED")]
    ActivityReplySubscribed(ActivityReplySubscribedNotification),
    #[serde(rename = "ACTIVITY_LIKE")]
    ActivityLike(ActivityLikeNotification),
    #[serde(rename = "ACTIVITY_REPLY_LIKE")]
    ActivityReplyLike(ActivityReplyLikeNotification),
    #[serde(rename = "THREAD_COMMENT_MENTION")]
    ThreadCommentMention(ThreadCommentMentionNotification),
    #[serde(rename = "THREAD_COMMENT_REPLY")]
    ThreadCommentReply(ThreadCommentReplyNotification),
    #[serde(rename = "THREAD_SUBSCRIBED")]
    ThreadSubscribed(ThreadCommentSubscribedNotification),
    #[serde(rename = "THREAD_COMMENT_LIKE")]
    ThreadCommentLike(ThreadCommentLikeNotification),
    #[serde(rename = "THREAD_LIKE")]
    ThreadLike(ThreadLikeNotification),
    #[serde(rename = "RELATED_MEDIA_ADDITION")]
    RelatedMediaAddition(RelatedMediaAdditionNotification),
    #[serde(rename = "MEDIA_DATA_CHANGE")]
    MediaDataChange(MediaDataChangeNotification),
    #[serde(rename = "MEDIA_MERGE")]
    MediaMerge(MediaMergeNotification),
    #[serde(rename = "MEDIA_DELETION")]
    MediaDeletion(MediaDeletionNotification),
}
