use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserStaffNameLanguage {
    RomajiWestern,
    Romaji,
    Native,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserTitleLanguage {
    Romaji,
    English,
    Native,
    RomajiStylised,
    EnglishStylised,
    NativeStylised,
}
