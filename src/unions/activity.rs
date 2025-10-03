use crate::objects::activity::{ListActivity, MessageActivity, TextActivity};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "__typename")]
pub enum ActivityUnion {
    TextActivity(TextActivity),
    ListActivity(ListActivity),
    MessageActivity(MessageActivity),
}
