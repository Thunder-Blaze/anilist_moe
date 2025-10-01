use serde::{Deserialize, Serialize};

use crate::objects::{common::PageInfo, media::Media};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiringProgression {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watching: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiringSchedule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "airingAt")]
    pub airing_at: i32,
    #[serde(rename = "timeUntilAiring")]
    pub time_until_airing: i32,
    pub episode: i32,
    #[serde(rename = "mediaId", skip_serializing_if = "Option::is_none")]
    pub media_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Box<Media>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiringScheduleConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<AiringScheduleEdge>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<AiringSchedule>>,
    #[serde(rename = "pageInfo", skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiringScheduleEdge {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<AiringSchedule>,
}
