use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RecommendationRating {
    #[default]
    NoRating,
    RateUp,
    RateDown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RecommendationSort {
    Id,
    IdDesc,
    Rating,
    RatingDesc,
}
