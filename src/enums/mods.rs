use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModActionType {
    Note,
    Ban,
    Delete,
    Edit,
    Expire,
    Report,
    Reset,
    Anon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModRole {
    Admin,
    LeadDeveloper,
    Developer,
    LeadCommunity,
    Community,
    DiscordCommunity,
    LeadAnimeData,
    AnimeData,
    LeadMangaData,
    MangaData,
    LeadSocialMedia,
    SocialMedia,
    Retired,
    CharacterData,
    StaffData,
}
