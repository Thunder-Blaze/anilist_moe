use serde::{Deserialize, Serialize};

use crate::objects::{common::PageInfo, media::MediaConnection};

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct Studio {
    pub id: Option<i32>,
    pub name: Option<String>,
    #[serde(rename = "isAnimationStudio")]
    pub is_animation_studio: Option<bool>,
    pub media: Option<MediaConnection>,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
    #[serde(rename = "isFavourite")]
    pub is_favourite: Option<bool>,
    pub favourites: Option<i32>,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct StudioConnection {
    pub edges: Option<Vec<StudioEdge>>,
    pub nodes: Option<Vec<Studio>>,
    #[serde(rename = "pageInfo")]
    pub page_info: Option<PageInfo>,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct StudioEdge {
    pub node: Option<Studio>,
    pub id: Option<i32>,
    #[serde(rename = "isMain")]
    pub is_main: Option<bool>,
    #[serde(rename = "favouriteOrder")]
    pub favourite_order: Option<i32>,
}