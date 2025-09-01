use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreadCommentSort {
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "ID_DESC")]
    IdDesc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreadSort {
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "ID_DESC")]
    IdDesc,
    #[serde(rename = "TITLE")]
    Title,
    #[serde(rename = "TITLE_DESC")]
    TitleDesc,
    #[serde(rename = "CREATED_AT")]
    CreatedAt,
    #[serde(rename = "CREATED_AT_DESC")]
    CreatedAtDesc,
    #[serde(rename = "UPDATED_AT")]
    UpdatedAt,
    #[serde(rename = "UPDATED_AT_DESC")]
    UpdatedAtDesc,
    #[serde(rename = "REPLIED_AT")]
    RepliedAt,
    #[serde(rename = "REPLIED_AT_DESC")]
    RepliedAtDesc,
    #[serde(rename = "REPLY_COUNT")]
    ReplyCount,
    #[serde(rename = "REPLY_COUNT_DESC")]
    ReplyCountDesc,
    #[serde(rename = "VIEW_COUNT")]
    ViewCount,
    #[serde(rename = "VIEW_COUNT_DESC")]
    ViewCountDesc,
    #[serde(rename = "IS_STICKY")]
    IsSticky,
    #[serde(rename = "SEARCH_MATCH")]
    SearchMatch,
}