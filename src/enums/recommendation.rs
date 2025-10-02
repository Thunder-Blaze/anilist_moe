use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RecommendationRating {
    NoRating,
    RateUp,
    RateDown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RecommendationSort {
    Id,
    IdDesc,
    Rating,
    RatingDesc,
}
