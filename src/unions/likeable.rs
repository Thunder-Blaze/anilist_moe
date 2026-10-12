use crate::objects::activity::{ActivityReply, ListActivity, MessageActivity, TextActivity};
use crate::objects::thread::{Thread, ThreadComment};
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "__typename")]
pub enum LikeableUnion {
    ListActivity(ListActivity),
    TextActivity(TextActivity),
    MessageActivity(MessageActivity),
    ActivityReply(ActivityReply),
    Thread(Thread),
    ThreadComment(Box<ThreadComment>),
    /// A `__typename` this version of the crate does not know yet.
    ///
    /// Only ever produced by deserialization.
    #[serde(other)]
    Unknown,
}
