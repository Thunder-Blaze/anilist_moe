use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivitySort {
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "ID_DESC")]
    IdDesc,
    #[serde(rename = "PINNED")]
    Pinned
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityType {
    #[serde(rename = "TEXT")]
    Text,
    #[serde(rename = "ANIME_LIST")]
    AnimeList,
    #[serde(rename = "MANGA_LIST")]
    MangaList,
    #[serde(rename = "MESSAGE")]
    Message,
    #[serde(rename = "MEDIA_LIST")]
    MediaList,
}