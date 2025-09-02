use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserSort {
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "ID_DESC")]
    IdDesc,
    #[serde(rename = "USERNAME")]
    Username,
    #[serde(rename = "USERNAME_DESC")]
    UsernameDesc,
    #[serde(rename = "WATCHED_TIME")]
    WatchedTime,
    #[serde(rename = "WATCHED_TIME_DESC")]
    WatchedTimeDesc,
    #[serde(rename = "CHAPTERS_READ")]
    ChaptersRead,
    #[serde(rename = "CHAPTERS_READ_DESC")]
    ChaptersReadDesc,
    #[serde(rename = "SEARCH_MATCH")]
    SearchMatch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserStaffNameLanguage {
    #[serde(rename = "ROMAJI_WESTERN")]
    RomajiWestern,
    #[serde(rename = "ROMAJI")]
    Romaji,
    #[serde(rename = "NATIVE")]
    Native,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserStatisticsSort {
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "ID_DESC")]
    IdDesc,
    #[serde(rename = "COUNT")]
    Count,
    #[serde(rename = "COUNT_DESC")]
    CountDesc,
    #[serde(rename = "PROGRESS")]
    Progress,
    #[serde(rename = "PROGRESS_DESC")]
    ProgressDesc,
    #[serde(rename = "MEAN_SCORE")]
    MeanScore,
    #[serde(rename = "MEAN_SCORE_DESC")]
    MeanScoreDesc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserTitleLanguage {
    #[serde(rename = "ROMAJI")]
    Romaji,
    #[serde(rename = "ENGLISH")]
    English,
    #[serde(rename = "NATIVE")]
    Native,
    #[serde(rename = "ROMAJI_STYLISED")]
    RomajiStylised,
    #[serde(rename = "ENGLISH_STYLISED")]
    EnglishStylised,
    #[serde(rename = "NATIVE_STYLISED")]
    NativeStylised,
}
