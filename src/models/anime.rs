//! # Anime Data Models
//!
//! This module contains data structures representing anime information
//! as returned by the AniList API.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anime {
    pub id: i32,
    pub title: Option<MediaTitle>,
    pub description: Option<String>,
    pub format: Option<MediaFormat>,
    pub status: Option<MediaStatus>,
    #[serde(rename = "startDate")]
    pub start_date: Option<FuzzyDate>,
    #[serde(rename = "endDate")]
    pub end_date: Option<FuzzyDate>,
    pub season: Option<MediaSeason>,
    #[serde(rename = "seasonYear")]
    pub season_year: Option<i32>,
    pub episodes: Option<i32>,
    pub duration: Option<i32>,
    pub genres: Option<Vec<String>>,
    #[serde(rename = "averageScore")]
    pub average_score: Option<i32>,
    #[serde(rename = "meanScore")]
    pub mean_score: Option<i32>,
    pub popularity: Option<i32>,
    pub favourites: Option<i32>,
    pub hashtag: Option<String>,
    #[serde(rename = "countryOfOrgin")]
    pub country_of_origin: Option<String>,
    #[serde(rename = "isAdult")]
    pub is_adult: Option<bool>,
    #[serde(rename = "nextAiringEpisode")]
    pub next_airing_episode: Option<AiringSchedule>,
    #[serde(rename = "coverImage")]
    pub cover_image: Option<MediaCoverImage>,
    #[serde(rename = "bannerImage")]
    pub banner_image: Option<String>,
    pub studios: Option<StudioConnection>,
    pub source: Option<MediaSource>,
    pub trailer: Option<MediaTrailer>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i32>,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaTitle {
    pub romaji: Option<String>,
    pub english: Option<String>,
    pub native: Option<String>,
    #[serde(rename = "userPreferred")]
    pub user_preferred: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzyDate {
    pub year: Option<i32>,
    pub month: Option<i32>,
    pub day: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaFormat {
    TV,
    #[serde(rename = "TV_SHORT")]
    TVSHORT,
    MOVIE,
    SPECIAL,
    OVA,
    ONA,
    MUSIC,
    MANGA,
    NOVEL,
    #[serde(rename = "ONE_SHOT")]
    ONESHOT,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaStatus {
    FINISHED,
    RELEASING,
    #[serde(rename = "NOT_YET_RELEASED")]
    NOTYETRELEASED,
    CANCELLED,
    HIATUS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaSeason {
    WINTER,
    SPRING,
    SUMMER,
    FALL,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaSource {
    ORIGINAL,
    MANGA,
    #[serde(rename = "LIGHT_NOVEL")]
    LIGHTNOVEL,
    #[serde(rename = "VISUAL_NOVEL")]
    VISUALNOVEL,
    #[serde(rename = "VIDEO_GAME")]
    VIDEOGAME,
    OTHER,
    NOVEL,
    DOUJINSHI,
    ANIME,
    #[serde(rename = "WEB_NOVEL")]
    WEBNOVEL,
    #[serde(rename = "LIVE_ACTION")]
    LIVEACTION,
    GAME,
    COMIC,
    #[serde(rename = "MULTIMEDIA_PROJECT")]
    MULTIMEDIAPROJECT,
    #[serde(rename = "PICTURE_BOOK")]
    PICTUREBOOK,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaCoverImage {
    #[serde(rename = "extraLarge")]
    pub extra_large: Option<String>,
    pub large: Option<String>,
    pub medium: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaTrailer {
    pub id: Option<String>,
    pub site: Option<String>,
    pub thumbnail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudioConnection {
    pub edges: Option<Vec<StudioEdge>>,
    pub nodes: Option<Vec<Studio>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudioEdge {
    pub node: Option<Studio>,
    #[serde(rename = "isMain")]
    pub is_main: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Studio {
    pub id: i32,
    pub name: String,
    #[serde(rename = "isAnimationStudio")]
    pub is_animation_studio: bool,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
}
