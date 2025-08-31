use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LikeableType {
    #[serde(rename = "THREAD")]
    Thread,
    #[serde(rename = "THREAD_COMMENT")]
    ThreadComment,
    #[serde(rename = "ACTIVITY")]
    Activity,
    #[serde(rename = "ACTIVITY_REPLY")]
    ActivityReply,
}