use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NotificationType {
    #[serde(rename = "ACTIVITY_MESSAGE")]
    ActivityMessage,
    #[serde(rename = "ACTIVITY_REPLY")]
    ActivityReply,
    #[serde(rename = "FOLLOWING")]
    Following,
    #[serde(rename = "ACTIVITY_MENTION")]
    ActivityMention,
    #[serde(rename = "THREAD_COMMENT_MENTION")]
    ThreadCommentMention,
    #[serde(rename = "THREAD_SUBSCRIBED")]
    ThreadSubscribed,
    #[serde(rename = "THREAD_COMMENT_REPLY")]
    ThreadCommentReply,
    #[serde(rename = "AIRING")]
    Airing,
    #[serde(rename = "ACTIVITY_LIKE")]
    ActivityLike,
    #[serde(rename = "ACTIVITY_REPLY_LIKE")]
    ActivityReplyLike,
    #[serde(rename = "THREAD_LIKE")]
    ThreadLike,
    #[serde(rename = "THREAD_COMMENT_LIKE")]
    ThreadCommentLike,
    #[serde(rename = "ACTIVITY_REPLY_SUBSCRIBED")]
    ActivityReplySubscribed,
    #[serde(rename = "RELATED_MEDIA_ADDITION")]
    RelatedMediaAddition,
    #[serde(rename = "MEDIA_DATA_CHANGE")]
    MediaDataChange,
    #[serde(rename = "MEDIA_MERGE")]
    MediaMerge,
    #[serde(rename = "MEDIA_DELETION")]
    MediaDeletion,
}
