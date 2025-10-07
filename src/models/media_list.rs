use super::{FuzzyDate, MediaCoverImage, MediaTitle};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaList {
    pub id: i32,
    pub user_id: i32,
    pub media_id: i32,
    pub status: Option<MediaListStatus>,
    pub score: Option<f64>,
    pub progress: Option<i32>,
    pub progress_volumes: Option<i32>,
    pub repeat: Option<i32>,
    pub priority: Option<i32>,
    pub private: Option<bool>,
    pub notes: Option<String>,
    pub hidden_from_status_lists: Option<bool>,
    pub custom_lists: Option<serde_json::Value>,
    pub advanced_scores: Option<serde_json::Value>,
    pub started_at: Option<FuzzyDate>,
    pub completed_at: Option<FuzzyDate>,
    pub updated_at: Option<i32>,
    pub created_at: Option<i32>,
    pub media: Option<MediaListMedia>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaListStatus {
    CURRENT,
    PLANNING,
    COMPLETED,
    DROPPED,
    PAUSED,
    REPEATING,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaListMedia {
    pub id: i32,
    pub title: Option<MediaTitle>,
    pub cover_image: Option<MediaCoverImage>,
    pub format: Option<String>,
    pub status: Option<String>,
    pub episodes: Option<i32>,
    pub chapters: Option<i32>,
    pub volumes: Option<i32>,
    pub season: Option<String>,
    pub season_year: Option<i32>,
    pub average_score: Option<i32>,
    pub genres: Option<Vec<String>>,
}
