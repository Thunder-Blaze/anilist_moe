use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FuzzyDate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    #[serde(rename = "currentPage", skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i32>,
    #[serde(rename = "lastPage", skip_serializing_if = "Option::is_none")]
    pub last_page: Option<i32>,
    #[serde(rename = "hasNextPage", skip_serializing_if = "Option::is_none")]
    pub has_next_page: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Json(serde_json::Value);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Deleted {
    pub deleted: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParsedMarkdown {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
}
