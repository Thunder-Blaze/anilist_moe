use serde::{Deserialize, Serialize};

use crate::{enums::{character::CharacterRole, submission::SubmissionStatus}, objects::{character::Character, common::PageInfo, staff::Staff, user::User}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSubmission {
    pub id: i32,
    pub character: Option<Character>,
    pub submission: Option<Character>,
    pub submitter: Option<User>,
    pub assignee: Option<User>,
    pub status: Option<SubmissionStatus>,
    pub notes: Option<String>,
    pub source: Option<String>,
    pub locked: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSubmissionConnection {
    pub edges: Option<Vec<CharacterSubmissionEdge>>,
    pub nodes: Option<Vec<CharacterSubmission>>,
    #[serde(rename = "pageInfo")]
    pub page_info: Option<PageInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSubmissionEdge {
    pub node: Option<CharacterSubmission>,
    pub role: Option<CharacterRole>,
    #[serde(rename = "voiceActors")]
    pub voice_actors: Option<Vec<Staff>>,
    #[serde(rename = "submittedVoiceActors")]
    pub submitted_voice_actors: Option<Vec<StaffSubmission>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffSubmission {
    pub id: i32,
    pub staff: Option<Staff>,
    pub submission: Option<Staff>,
    pub submitter: Option<User>,
    pub assignee: Option<User>,
    pub status: Option<SubmissionStatus>,
    pub notes: Option<String>,
    pub source: Option<String>,
    pub locked: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<i32>,
}