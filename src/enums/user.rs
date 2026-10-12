use crate::options::SortSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserStaffNameLanguage {
    RomajiWestern,
    Romaji,
    Native,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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
pub enum UserTitleLanguage {
    Romaji,
    English,
    Native,
    RomajiStylised,
    EnglishStylised,
    NativeStylised,
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
