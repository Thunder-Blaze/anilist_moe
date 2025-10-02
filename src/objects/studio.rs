use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::objects::{common::PageInfo, media::MediaConnection};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Studio {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub is_animation_studio: Option<bool>,
    pub media: Option<MediaConnection>,
    pub site_url: Option<String>,
    pub is_favourite: Option<bool>,
    pub favourites: Option<i32>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioConnection {
    pub edges: Option<Vec<StudioEdge>>,
    pub nodes: Option<Vec<Studio>>,
    pub page_info: Option<PageInfo>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioEdge {
    pub node: Option<Studio>,
    pub id: Option<i32>,
    pub is_main: Option<bool>,
    pub favourite_order: Option<i32>,
}
