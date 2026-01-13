use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    enums::{
        character::CharacterRole,
        external_link::ExternalLinkType,
        media::{
            CountryCode, MediaFormat, MediaRankType, MediaRelation, MediaSeason, MediaSource,
            MediaStatus, MediaType,
        },
        submission::SubmissionStatus,
    },
    objects::{
        airing::{AiringSchedule, AiringScheduleConnection},
        character::{Character, CharacterConnection},
        common::{FuzzyDate, Json, PageInfo},
        media_list::MediaList,
        recommendation::RecommendationConnection,
        review::ReviewConnection,
        staff::{Staff, StaffConnection, StaffEdge, StaffRoleType},
        stats::MediaStats,
        studio::{Studio, StudioConnection, StudioEdge},
        user::User,
    },
};

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub id: Option<i32>,
    pub id_mal: Option<i32>,
    pub title: Option<MediaTitle>,
    #[serde(rename = "type")]
    pub media_type: Option<MediaType>,
    pub format: Option<MediaFormat>,
    pub status: Option<MediaStatus>,
    pub description: Option<String>,
    pub start_date: Option<FuzzyDate>,
    pub end_date: Option<FuzzyDate>,
    pub season: Option<MediaSeason>,
    pub season_year: Option<i32>,
    pub season_int: Option<i32>,
    pub episodes: Option<i32>,
    pub duration: Option<i32>,
    pub chapters: Option<i32>,
    pub volumes: Option<i32>,
    pub country_of_origin: Option<CountryCode>,
    pub is_licensed: Option<bool>,
    pub source: Option<MediaSource>,
    pub hashtag: Option<String>,
    pub trailer: Option<MediaTrailer>,
    pub updated_at: Option<i32>,
    pub cover_image: Option<MediaCoverImage>,
    pub banner_image: Option<String>,
    pub genres: Option<Vec<String>>,
    pub synonyms: Option<Vec<String>>,
    pub average_score: Option<i32>,
    pub mean_score: Option<i32>,
    pub popularity: Option<i32>,
    pub is_locked: Option<bool>,
    pub trending: Option<i32>,
    pub favourites: Option<i32>,
    pub tags: Option<Vec<MediaTag>>,
    pub relations: Option<MediaConnection>,
    pub characters: Option<CharacterConnection>,
    pub staff: Option<StaffConnection>,
    pub studios: Option<StudioConnection>,
    #[serde(rename = "fullStudios")]
    pub studios_full: Option<StudioConnection>,
    pub is_favourite: Option<bool>,
    pub is_favourite_blocked: Option<bool>,
    pub is_adult: Option<bool>,
    pub next_airing_episode: Option<AiringSchedule>,
    pub airing_schedule: Option<AiringScheduleConnection>,
    pub trends: Option<MediaTrendConnection>,
    pub external_links: Option<Vec<MediaExternalLink>>,
    pub streaming_episodes: Option<Vec<MediaStreamingEpisode>>,
    pub rankings: Option<Vec<MediaRank>>,
    pub media_list_entry: Option<Box<MediaList>>,
    pub reviews: Option<ReviewConnection>,
    pub recommendations: Option<RecommendationConnection>,
    pub stats: Option<MediaStats>,
    pub site_url: Option<String>,
    pub auto_create_forum_thread: Option<bool>,
    pub is_recommendation_blocked: Option<bool>,
    pub is_review_blocked: Option<bool>,
    pub mod_notes: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaCharacter {
    pub id: Option<i32>,
    pub role: Option<CharacterRole>,
    pub role_notes: Option<String>,
    pub dub_group: Option<String>,
    pub character_name: Option<String>,
    pub character: Option<Character>,
    pub voice_actor: Option<Staff>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaConnection {
    pub edges: Option<Vec<MediaEdge>>,
    pub nodes: Option<Vec<Media>>,
    pub page_info: Option<PageInfo>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaCoverImage {
    pub extra_large: Option<String>,
    pub large: Option<String>,
    pub medium: Option<String>,
    pub color: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaEdge {
    pub node: Option<Media>,
    pub id: Option<i32>,
    pub relation_type: Option<MediaRelation>,
    pub is_main_studio: Option<bool>,
    pub characters: Option<Vec<Character>>,
    pub character_role: Option<CharacterRole>,
    pub character_name: Option<String>,
    pub role_notes: Option<String>,
    pub dub_group: Option<String>,
    pub staff_role: Option<String>,
    pub voice_actors: Option<Vec<Staff>>,
    pub voice_actor_roles: Option<Vec<StaffRoleType>>,
    pub favourite_order: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaExternalLink {
    pub id: i32,
    pub url: Option<String>,
    pub site: Option<String>,
    pub site_id: Option<i32>,
    #[serde(rename = "type")]
    pub link_type: Option<ExternalLinkType>,
    pub language: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub notes: Option<String>,
    pub is_disabled: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaRank {
    pub id: i32,
    pub rank: Option<i32>,
    #[serde(rename = "type")]
    pub rank_type: Option<MediaRankType>,
    pub format: Option<MediaFormat>,
    pub year: Option<i32>,
    pub season: Option<MediaSeason>,
    pub all_time: Option<bool>,
    pub context: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaStreamingEpisode {
    pub title: Option<String>,
    pub thumbnail: Option<String>,
    pub url: Option<String>,
    pub site: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaSubmission {
    pub id: i32,
    pub submitter: Option<User>,
    pub assignee: Option<User>,
    pub status: Option<SubmissionStatus>,
    pub submitter_stats: Option<Json>,
    pub notes: Option<String>,
    pub source: Option<String>,
    pub changes: Option<Vec<String>>,
    pub locked: Option<bool>,
    pub media: Option<Media>,
    pub submission: Option<Media>,
    pub characters: Option<Vec<MediaSubmissionComparison>>,
    pub staff: Option<Vec<MediaSubmissionComparison>>,
    pub studios: Option<Vec<MediaSubmissionComparison>>,
    pub relations: Option<Vec<MediaEdge>>,
    pub external_links: Option<Vec<MediaSubmissionComparison>>,
    pub created_at: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaSubmissionComparison {
    pub submission: Option<MediaSubmissionEdge>,
    pub character: Option<MediaCharacter>,
    pub staff: Option<StaffEdge>,
    pub studio: Option<StudioEdge>,
    pub external_link: Option<MediaExternalLink>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaSubmissionEdge {
    pub id: i32,
    pub character_role: Option<CharacterRole>,
    pub staff_role: Option<String>,
    pub role_notes: Option<String>,
    pub dub_group: Option<String>,
    pub character_name: Option<String>,
    pub is_main: Option<bool>,
    pub character: Option<Character>,
    pub character_submission: Option<Character>,
    pub voice_actor: Option<Staff>,
    pub voice_actor_submission: Option<Staff>,
    pub staff: Option<Staff>,
    pub staff_submission: Option<Staff>,
    pub studio: Option<Studio>,
    pub external_link: Option<MediaExternalLink>,
    pub media: Option<Media>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaTag {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub rank: Option<i32>,
    pub is_general_spoiler: Option<bool>,
    pub is_media_spoiler: Option<bool>,
    pub is_adult: Option<bool>,
    pub user_id: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaTitle {
    pub romaji: Option<String>,
    pub english: Option<String>,
    pub native: Option<String>,
    pub user_preferred: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaTrailer {
    pub id: Option<String>,
    pub site: Option<String>,
    pub thumbnail: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaTrend {
    pub media_id: Option<i32>,
    pub date: Option<i32>,
    pub trending: Option<i32>,
    pub average_score: Option<i32>,
    pub popularity: Option<i32>,
    pub in_progress: Option<i32>,
    pub releasing: Option<bool>,
    pub episode: Option<i32>,
    pub media: Option<Media>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaTrendConnection {
    pub edges: Option<Vec<MediaTrendEdge>>,
    pub nodes: Option<Vec<MediaTrend>>,
    pub page_info: Option<PageInfo>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaTrendEdge {
    pub node: Option<MediaTrend>,
}
