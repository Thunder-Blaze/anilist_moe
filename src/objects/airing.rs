use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::objects::{common::PageInfo, media::Media};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiringProgression {
    pub episode: Option<f32>,
    pub progress: Option<f32>,
    pub watching: Option<i32>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiringSchedule {
    pub id: Option<i32>,
    pub airing_at: Option<i32>,
    pub time_until_airing: Option<i32>,
    pub episode: Option<i32>,
    pub media_id: Option<i32>,
    pub media: Option<Box<Media>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiringScheduleConnection {
    pub edges: Option<Vec<AiringScheduleEdge>>,
    pub nodes: Option<Vec<AiringSchedule>>,
    pub page_info: Option<PageInfo>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiringScheduleEdge {
    pub id: Option<i32>,
    pub node: Option<AiringSchedule>,
}
