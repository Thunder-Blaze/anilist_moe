use serde::{Deserialize, Serialize};

use crate::objects::{common::PageInfo, media::Media};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiringProgression {
    pub episode: Option<f32>,
    pub progress: Option<f32>,
    pub watching: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiringSchedule {
    pub id: i32,
    #[serde(rename = "airingAt")]
    pub airing_at: i32,
    #[serde(rename = "timeUntilAiring")]
    pub time_until_airing: i32,
    pub episode: i32,
    #[serde(rename = "mediaId")]
    pub media_id: i32,
    pub media: Box<Option<Media>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiringScheduleConnection {
    pub edges: Option<Vec<AiringScheduleEdge>>,
    pub nodes: Option<Vec<AiringSchedule>>,
    #[serde(rename = "pageInfo")]
    pub page_info: Option<PageInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiringScheduleEdge {
    pub id: Option<i32>,
    pub node: Option<AiringSchedule>,
}