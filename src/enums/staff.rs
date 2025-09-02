use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StaffLanguage {
    #[serde(rename = "JAPANESE")]
    Japanese,
    #[serde(rename = "ENGLISH")]
    English,
    #[serde(rename = "KOREAN")]
    Korean,
    #[serde(rename = "ITALIAN")]
    Italian,
    #[serde(rename = "SPANISH")]
    Spanish,
    #[serde(rename = "PORTUGUESE")]
    Portuguese,
    #[serde(rename = "FRENCH")]
    French,
    #[serde(rename = "GERMAN")]
    German,
    #[serde(rename = "HEBREW")]
    Hebrew,
    #[serde(rename = "HUNGARIAN")]
    Hungarian,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StaffSort {
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "ID_DESC")]
    IdDesc,
    #[serde(rename = "ROLE")]
    Role,
    #[serde(rename = "ROLE_DESC")]
    RoleDesc,
    #[serde(rename = "LANGUAGE")]
    Language,
    #[serde(rename = "LANGUAGE_DESC")]
    LanguageDesc,
    #[serde(rename = "SEARCH_MATCH")]
    SearchMatch,
    #[serde(rename = "FAVOURITES")]
    Favourites,
    #[serde(rename = "FAVOURITES_DESC")]
    FavouritesDesc,
    #[serde(rename = "RELEVANCE")]
    Relevance,
}
