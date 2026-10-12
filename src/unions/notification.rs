use crate::objects::notification::{
    ActivityLikeNotification, ActivityMentionNotification, ActivityMessageNotification,
    ActivityReplyLikeNotification, ActivityReplyNotification, ActivityReplySubscribedNotification,
    AiringNotification, CharacterSubmissionUpdateNotification, FollowingNotification,
    MediaDataChangeNotification, MediaDeletionNotification, MediaMergeNotification,
    MediaSubmissionUpdateNotification, RelatedMediaAdditionNotification,
    StaffSubmissionUpdateNotification, ThreadCommentLikeNotification,
    ThreadCommentMentionNotification, ThreadCommentReplyNotification,
    ThreadCommentSubscribedNotification, ThreadLikeNotification,
};
use serde::{Deserialize, Serialize};

#[non_exhaustive]
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
    MediaSubmissionUpdate(MediaSubmissionUpdateNotification),
    StaffSubmissionUpdate(StaffSubmissionUpdateNotification),
    CharacterSubmissionUpdate(CharacterSubmissionUpdateNotification),
    /// A `__typename` this version of the crate does not know yet.
    ///
    /// Only ever produced by deserialization.
    #[serde(other)]
    Unknown,
}
