use crate::options::SortSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivitySort {
    Id,
    IdDesc,
    Pinned,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityType {
    Text,
    AnimeList,
    MangaList,
    Message,
    MediaList,
}

impl SortSpec for ActivitySort {
    fn id_tiebreaker() -> Self {
        Self::Id
    }
}
