use serde::{Deserialize, Serialize};
use crate::objects::activity::{TextActivity, ListActivity, MessageActivity};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityUnion {
    TextActivity(TextActivity),
    ListActivity(ListActivity),
    MessageActivity(MessageActivity),
}
