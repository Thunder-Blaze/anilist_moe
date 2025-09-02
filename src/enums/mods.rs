use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModActionType {
    #[serde(rename = "NOTE")]
    Note,
    #[serde(rename = "BAN")]
    Ban,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "EDIT")]
    Edit,
    #[serde(rename = "EXPIRE")]
    Expire,
    #[serde(rename = "REPORT")]
    Report,
    #[serde(rename = "RESET")]
    Reset,
    #[serde(rename = "ANON")]
    Anon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModRole {
    #[serde(rename = "ADMIN")]
    Admin,
    #[serde(rename = "LEAD_DEVELOPER")]
    LeadDeveloper,
    #[serde(rename = "DEVELOPER")]
    Developer,
    #[serde(rename = "LEAD_COMMUNITY")]
    LeadCommunity,
    #[serde(rename = "COMMUNITY")]
    Community,
    #[serde(rename = "DISCORD_COMMUNITY")]
    DiscordCommunity,
    #[serde(rename = "LEAD_ANIME_DATA")]
    LeadAnimeData,
    #[serde(rename = "ANIME_DATA")]
    AnimeData,
    #[serde(rename = "LEAD_MANGA_DATA")]
    LeadMangaData,
    #[serde(rename = "MANGA_DATA")]
    MangaData,
    #[serde(rename = "LEAD_SOCIAL_MEDIA")]
    LeadSocialMedia,
    #[serde(rename = "SOCIAL_MEDIA")]
    SocialMedia,
    #[serde(rename = "RETIRED")]
    Retired,
    #[serde(rename = "CHARACTER_DATA")]
    CharacterData,
    #[serde(rename = "STAFF_DATA")]
    StaffData,
}
