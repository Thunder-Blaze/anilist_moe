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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<StaffName>,
    #[serde(rename = "languageV2", skip_serializing_if = "Option::is_none")]
    pub language_v2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<StaffImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "primaryOccupations", skip_serializing_if = "Option::is_none")]
    pub primary_occupations: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(rename = "dateOfBirth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<FuzzyDate>,
    #[serde(rename = "dateOfDeath", skip_serializing_if = "Option::is_none")]
    pub date_of_death: Option<FuzzyDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<i32>,
    #[serde(rename = "yearsActive", skip_serializing_if = "Option::is_none")]
    pub years_active: Option<Vec<i32>>,
    #[serde(rename = "homeTown", skip_serializing_if = "Option::is_none")]
    pub home_town: Option<String>,
    #[serde(rename = "bloodType", skip_serializing_if = "Option::is_none")]
    pub blood_type: Option<String>,
    #[serde(rename = "isFavourite", skip_serializing_if = "Option::is_none")]
    pub is_favourite: Option<bool>,
    #[serde(rename = "isFavouriteBlocked", skip_serializing_if = "Option::is_none")]
    pub is_favourite_blocked: Option<bool>,
    #[serde(rename = "siteUrl", skip_serializing_if = "Option::is_none")]
    pub site_url: Option<String>,
    #[serde(rename = "staffMedia", skip_serializing_if = "Option::is_none")]
    pub staff_media: Option<MediaConnection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characters: Option<CharacterConnection>,
    #[serde(rename = "characterMedia", skip_serializing_if = "Option::is_none")]
    pub character_media: Option<MediaConnection>,
    pub staff: Box<Option<Staff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitter: Option<User>,
    #[serde(rename = "submissionStatus", skip_serializing_if = "Option::is_none")]
    pub submission_status: Option<i32>,
    #[serde(rename = "submissionNotes", skip_serializing_if = "Option::is_none")]
    pub submission_notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favourites: Option<i32>,
    #[serde(rename = "modNotes", skip_serializing_if = "Option::is_none")]
    pub mod_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<StaffEdge>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Staff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffEdge {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<Staff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "favouriteOrder", skip_serializing_if = "Option::is_none")]
    pub favourite_order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffName {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternative: Option<Vec<String>>,
    #[serde(rename = "userPreferred", skip_serializing_if = "Option::is_none")]
    pub user_preferred: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffRoleType {
    #[serde(rename = "voiceActor", skip_serializing_if = "Option::is_none")]
    pub voice_actor: Option<Staff>,
    #[serde(rename = "roleNotes", skip_serializing_if = "Option::is_none")]
    pub role_notes: Option<String>,
    #[serde(rename = "dubGroup", skip_serializing_if = "Option::is_none")]
    pub dub_group: Option<String>,
}
