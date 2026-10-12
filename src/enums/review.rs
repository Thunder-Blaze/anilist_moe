use crate::options::SortSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReviewRating {
    #[default]
    NoVote,
    UpVote,
    DownVote,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReviewSort {
    Id,
    IdDesc,
    Score,
    ScoreDesc,
    Rating,
    RatingDesc,
    CreatedAt,
    CreatedAtDesc,
    UpdatedAt,
    UpdatedAtDesc,
}

impl SortSpec for ReviewSort {
    fn id_tiebreaker() -> Self {
        Self::Id
    }
}
