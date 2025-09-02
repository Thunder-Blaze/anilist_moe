use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExternalLinkMediaType {
    #[serde(rename = "ANIME")]
    Anime,
    #[serde(rename = "MANGA")]
    Manga,
    #[serde(rename = "STAFF")]
    Staff,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExternalLinkType {
    #[serde(rename = "INFO")]
    Info,
    #[serde(rename = "STREAMING")]
    Streaming,
    #[serde(rename = "SOCIAL")]
    Social,
}
