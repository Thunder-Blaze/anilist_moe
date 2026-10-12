use crate::options::SortSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum StaffLanguage {
    Japanese,
    English,
    Korean,
    Italian,
    Spanish,
    Portuguese,
    French,
    German,
    Hebrew,
    Hungarian,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum StaffSort {
    Id,
    IdDesc,
    Role,
    RoleDesc,
    Language,
    LanguageDesc,
    SearchMatch,
    Favourites,
    FavouritesDesc,
    Relevance,
}

impl SortSpec for StaffSort {
    fn id_tiebreaker() -> Self {
        Self::Id
    }
}
