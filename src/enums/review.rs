use crate::options::SortSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum ReviewRating {
    #[default]
    NoVote,
    UpVote,
    DownVote,
    /// A value the API returned that this version of the crate does not know yet.
    ///
    /// Only ever produced by deserialization; never send it as input.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum ReviewSort {
    Id,
    IdDesc,
    Score,
    ScoreDesc,
    Rating,
    RatingDesc,
    #[deprecated(note = "the API deprecated it: use `Id` instead")]
    CreatedAt,
    #[deprecated(note = "the API deprecated it: use `IdDesc` instead")]
    CreatedAtDesc,
    UpdatedAt,
    UpdatedAtDesc,
}

impl SortSpec for ReviewSort {
    fn id_tiebreaker() -> Self {
        Self::Id
    }
}
