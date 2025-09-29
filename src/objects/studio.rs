use serde::{Deserialize, Serialize};

use crate::objects::{common::PageInfo, media::MediaConnection};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Studio {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isAnimationStudio", skip_serializing_if = "Option::is_none")]
    pub is_animation_studio: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<MediaConnection>,
    #[serde(rename = "siteUrl", skip_serializing_if = "Option::is_none")]
    pub site_url: Option<String>,
    #[serde(rename = "isFavourite", skip_serializing_if = "Option::is_none")]
    pub is_favourite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favourites: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudioConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<StudioEdge>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Studio>>,
    #[serde(rename = "pageInfo", skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudioEdge {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<Studio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "isMain", skip_serializing_if = "Option::is_none")]
    pub is_main: Option<bool>,
    #[serde(rename = "favouriteOrder", skip_serializing_if = "Option::is_none")]
    pub favourite_order: Option<i32>,
}
