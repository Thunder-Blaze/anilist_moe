use crate::objects::activity::{ListActivity, MessageActivity, TextActivity};
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "__typename")]
pub enum ActivityUnion {
    TextActivity(TextActivity),
    ListActivity(ListActivity),
    MessageActivity(MessageActivity),
    /// A `__typename` this version of the crate does not know yet.
    ///
    /// Only ever produced by deserialization.
    #[serde(other)]
    Unknown,
}
