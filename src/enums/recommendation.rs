use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationRating {
    #[serde(rename = "NO_RATING")]
    NoRating,
    #[serde(rename = "RATE_UP")]
    RateUp,
    #[serde(rename = "RATE_DOWN")]
    RateDown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationSort {
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "ID_DESC")]
    IdDesc,
    #[serde(rename = "RATING")]
    Rating,
    #[serde(rename = "RATING_DESC")]
    RatingDesc,
}
