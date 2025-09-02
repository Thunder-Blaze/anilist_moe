use serde::{Deserialize, Serialize};

use crate::objects::{
    character::CharacterConnection,
    common::{FuzzyDate, PageInfo},
    media::MediaConnection,
    user::User,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Staff {
    pub id: i32,
    pub name: Option<StaffName>,
    #[serde(rename = "languageV2")]
    pub language_v2: Option<String>,
    pub image: Option<StaffImage>,
    pub description: Option<String>,
    #[serde(rename = "primaryOccupations")]
    pub primary_occupations: Option<Vec<String>>,
    pub gender: Option<String>,
    #[serde(rename = "dateOfBirth")]
    pub date_of_birth: Option<FuzzyDate>,
    #[serde(rename = "dateOfDeath")]
    pub date_of_death: Option<FuzzyDate>,
    pub age: Option<i32>,
    #[serde(rename = "yearsActive")]
    pub years_active: Option<Vec<i32>>,
    #[serde(rename = "homeTown")]
    pub home_town: Option<String>,
    #[serde(rename = "bloodType")]
    pub blood_type: Option<String>,
    #[serde(rename = "isFavourite")]
    pub is_favourite: Option<bool>,
    #[serde(rename = "isFavouriteBlocked")]
    pub is_favourite_blocked: Option<bool>,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
    #[serde(rename = "staffMedia")]
    pub staff_media: Option<MediaConnection>,
    pub characters: Option<CharacterConnection>,
    #[serde(rename = "characterMedia")]
    pub character_media: Option<MediaConnection>,
    pub staff: Box<Option<Staff>>,
    pub submitter: Option<User>,
    #[serde(rename = "submissionStatus")]
    pub submission_status: Option<i32>,
    #[serde(rename = "submissionNotes")]
    pub submission_notes: Option<String>,
    pub favourites: Option<i32>,
    #[serde(rename = "modNotes")]
    pub mod_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffConnection {
    pub edges: Option<Vec<StaffEdge>>,
    pub nodes: Option<Vec<Staff>>,
    pub page_info: Option<PageInfo>,
    pub total_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffEdge {
    pub node: Option<Staff>,
    pub id: Option<i32>,
    pub role: Option<String>,
    #[serde(rename = "favouriteOrder")]
    pub favourite_order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffImage {
    pub large: Option<String>,
    pub medium: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffName {
    pub first: Option<String>,
    pub middle: Option<String>,
    pub last: Option<String>,
    pub full: Option<String>,
    pub native: Option<String>,
    pub alternative: Option<Vec<String>>,
    #[serde(rename = "userPreferred")]
    pub user_preferred: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffRoleType {
    #[serde(rename = "voiceActor")]
    pub voice_actor: Option<Staff>,
    #[serde(rename = "roleNotes")]
    pub role_notes: Option<String>,
    #[serde(rename = "dubGroup")]
    pub dub_group: Option<String>,
}
