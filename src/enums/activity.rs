use serde::{Deserialize, Serialize};

#[derive(Debug,   Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivitySort {
    Id,
    IdDesc,
    Pinned,
}

#[derive(Debug,   Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityType {
    Text,
    AnimeList,
    MangaList,
    Message,
    MediaList,
}
