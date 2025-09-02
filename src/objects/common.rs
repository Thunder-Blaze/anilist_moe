use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FuzzyDate {
    pub year: Option<i32>,
    pub month: Option<i32>,
    pub day: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageInfo {
    total: Option<i32>,
    #[serde(rename = "perPage")]
    per_page: Option<i32>,
    #[serde(rename = "currentPage")]
    current_page: Option<i32>,
    #[serde(rename = "lastPage")]
    last_page: Option<i32>,
    #[serde(rename = "hasNextPage")]
    has_next_page: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Json(serde_json::Value);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Deleted {
    pub deleted: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParsedMarkdown {
    pub html: Option<String>,
}