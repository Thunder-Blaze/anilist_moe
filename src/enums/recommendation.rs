use crate::options::SortSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum RecommendationRating {
    #[default]
    NoRating,
    RateUp,
    RateDown,
    /// A value the API returned that this version of the crate does not know yet.
    ///
    /// Only ever produced by deserialization; never send it as input.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum RecommendationSort {
    Id,
    IdDesc,
    Rating,
    RatingDesc,
}

impl SortSpec for RecommendationSort {
    fn id_tiebreaker() -> Self {
        Self::Id
    }
}
