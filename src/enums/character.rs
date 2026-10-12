use crate::options::SortSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum CharacterRole {
    Main,
    Supporting,
    Background,
    /// A value the API returned that this version of the crate does not know yet.
    ///
    /// Only ever produced by deserialization; never send it as input.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum CharacterSort {
    Id,
    IdDesc,
    Role,
    RoleDesc,
    SearchMatch,
    Favourites,
    FavouritesDesc,
    Relevance,
}

impl SortSpec for CharacterSort {
    fn id_tiebreaker() -> Self {
        Self::Id
    }
}
