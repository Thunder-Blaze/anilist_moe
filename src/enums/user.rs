use crate::options::SortSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum UserSort {
    Id,
    IdDesc,
    Username,
    UsernameDesc,
    WatchedTime,
    WatchedTimeDesc,
    ChaptersRead,
    ChaptersReadDesc,
    SearchMatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum UserStaffNameLanguage {
    RomajiWestern,
    Romaji,
    Native,
    /// A value the API returned that this version of the crate does not know yet.
    ///
    /// Only ever produced by deserialization; never send it as input.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum UserStatisticsSort {
    Id,
    IdDesc,
    Count,
    CountDesc,
    Progress,
    ProgressDesc,
    MeanScore,
    MeanScoreDesc,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum UserTitleLanguage {
    Romaji,
    English,
    Native,
    RomajiStylised,
    EnglishStylised,
    NativeStylised,
    /// A value the API returned that this version of the crate does not know yet.
    ///
    /// Only ever produced by deserialization; never send it as input.
    #[serde(other)]
    Unknown,
}

impl SortSpec for UserSort {
    fn id_tiebreaker() -> Self {
        Self::Id
    }
}

impl SortSpec for UserStatisticsSort {
    fn id_tiebreaker() -> Self {
        Self::Id
    }
}
