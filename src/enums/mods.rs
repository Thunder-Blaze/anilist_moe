use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum ModActionType {
    Note,
    Ban,
    Delete,
    Edit,
    Expire,
    Report,
    Reset,
    Anon,
    /// A value the API returned that this version of the crate does not know yet.
    ///
    /// Only ever produced by deserialization; never send it as input.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
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
    /// A value the API returned that this version of the crate does not know yet.
    ///
    /// Only ever produced by deserialization; never send it as input.
    #[serde(other)]
    Unknown,
}
