use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StudioSort {
    #[serde(rename = "ID")]
    Id,	
    #[serde(rename = "ID_DESC")]
    IdDesc,
    #[serde(rename = "NAME")]
    Name,
    #[serde(rename = "NAME_DESC")]
    NameDesc,
    #[serde(rename = "SEARCH_MATCH")]
    SearchMatch,
    #[serde(rename = "FAVOURITES")]
    Favourites,
    #[serde(rename = "FAVOURITES_DESC")]
    FavouritesDesc,
}
