use crate::objects::activity::{ListActivity, MessageActivity, TextActivity};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityUnion {
    TextActivity(TextActivity),
    ListActivity(ListActivity),
    MessageActivity(MessageActivity),
}
