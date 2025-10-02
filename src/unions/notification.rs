use crate::objects::notification::{
    ActivityLikeNotification, ActivityMentionNotification, ActivityMessageNotification,
    ActivityReplyLikeNotification, ActivityReplyNotification, ActivityReplySubscribedNotification,
    AiringNotification, FollowingNotification, MediaDataChangeNotification,
    MediaDeletionNotification, MediaMergeNotification, RelatedMediaAdditionNotification,
    ThreadCommentLikeNotification, ThreadCommentMentionNotification,
    ThreadCommentReplyNotification, ThreadCommentSubscribedNotification, ThreadLikeNotification,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationUnion {
    Airing(AiringNotification),
    Following(FollowingNotification),
    ActivityMessage(ActivityMessageNotification),
    ActivityMention(ActivityMentionNotification),
    ActivityReply(ActivityReplyNotification),
    ActivityReplySubscribed(ActivityReplySubscribedNotification),
    ActivityLike(ActivityLikeNotification),
    ActivityReplyLike(ActivityReplyLikeNotification),
    ThreadCommentMention(ThreadCommentMentionNotification),
    ThreadCommentReply(ThreadCommentReplyNotification),
    ThreadSubscribed(ThreadCommentSubscribedNotification),
    ThreadCommentLike(ThreadCommentLikeNotification),
    ThreadLike(ThreadLikeNotification),
    RelatedMediaAddition(RelatedMediaAdditionNotification),
    MediaDataChange(MediaDataChangeNotification),
    MediaMerge(MediaMergeNotification),
    MediaDeletion(MediaDeletionNotification),
}
