use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReviewRating {
    #[serde(rename = "NO_VOTE")]
    NoVote,
    #[serde(rename = "UP_VOTE")]
    UpVote,
    #[serde(rename = "DOWN_VOTE")]
    DownVote,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReviewSort {
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "ID_DESC")]
    IdDesc,
    #[serde(rename = "SCORE")]
    Score,
    #[serde(rename = "SCORE_DESC")]
    ScoreDesc,
    #[serde(rename = "RATING")]
    Rating,
    #[serde(rename = "RATING_DESC")]
    RatingDesc,
    #[serde(rename = "CREATED_AT")]
    CreatedAt,
    #[serde(rename = "CREATED_AT_DESC")]
    CreatedAtDesc,
    #[serde(rename = "UPDATED_AT")]
    UpdatedAt,
    #[serde(rename = "UPDATED_AT_DESC")]
    UpdatedAtDesc,
}
