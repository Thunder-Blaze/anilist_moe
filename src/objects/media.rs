use serde::{Deserialize, Serialize};

use crate::{
    enums::{
        character::CharacterRole,
        external_link::ExternalLinkType,
        media::{
            CountryCode, MediaFormat, MediaRankType, MediaRelation, MediaSeason, MediaSource,
            MediaStatus, MediaType,
        },
        notification::NotificationType,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Media {
    pub id: i32,
    #[serde(rename = "idMal", skip_serializing_if = "Option::is_none")]
    pub id_mal: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<MediaTitle>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<MediaType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<MediaFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MediaStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<FuzzyDate>,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<FuzzyDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<MediaSeason>,
    #[serde(rename = "seasonYear", skip_serializing_if = "Option::is_none")]
    pub season_year: Option<i32>,
    #[serde(rename = "seasonInt", skip_serializing_if = "Option::is_none")]
    pub season_int: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episodes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapters: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<i32>,
    #[serde(rename = "countryOfOrigin", skip_serializing_if = "Option::is_none")]
    pub country_of_origin: Option<CountryCode>,
    #[serde(rename = "isLicensed", skip_serializing_if = "Option::is_none")]
    pub is_licensed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<MediaSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashtag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailer: Option<MediaTrailer>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i32>,
    #[serde(rename = "coverImage", skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<MediaCoverImage>,
    #[serde(rename = "bannerImage", skip_serializing_if = "Option::is_none")]
    pub banner_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<Vec<String>>,
    #[serde(rename = "averageScore", skip_serializing_if = "Option::is_none")]
    pub average_score: Option<i32>,
    #[serde(rename = "meanScore", skip_serializing_if = "Option::is_none")]
    pub mean_score: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub popularity: Option<i32>,
    #[serde(rename = "isLocked", skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trending: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favourites: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<MediaTag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relations: Option<MediaConnection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characters: Option<CharacterConnection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staff: Option<StaffConnection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studios: Option<StudioConnection>,
    #[serde(rename = "isFavourite", skip_serializing_if = "Option::is_none")]
    pub is_favourite: Option<bool>,
    #[serde(rename = "isFavouriteBlocked", skip_serializing_if = "Option::is_none")]
    pub is_favourite_blocked: Option<bool>,
    #[serde(rename = "isAdult", skip_serializing_if = "Option::is_none")]
    pub is_adult: Option<bool>,
    #[serde(rename = "nextAiringEpisode", skip_serializing_if = "Option::is_none")]
    pub next_airing_episode: Option<AiringSchedule>,
    #[serde(rename = "airingSchedule", skip_serializing_if = "Option::is_none")]
    pub airing_schedule: Option<AiringScheduleConnection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trends: Option<MediaTrendConnection>,
    #[serde(rename = "externalLinks", skip_serializing_if = "Option::is_none")]
    pub external_links: Option<Vec<MediaExternalLink>>,
    #[serde(rename = "streamingEpisodes", skip_serializing_if = "Option::is_none")]
    pub streaming_episodes: Option<Vec<MediaStreamingEpisode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rankings: Option<Vec<MediaRank>>,
    #[serde(rename = "mediaListEntry", skip_serializing_if = "Option::is_none")]
    pub media_list_entry: Option<Box<MediaList>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviews: Option<ReviewConnection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<RecommendationConnection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<MediaStats>,
    #[serde(rename = "siteUrl", skip_serializing_if = "Option::is_none")]
    pub site_url: Option<String>,
    #[serde(rename = "autoCreateForumThread", skip_serializing_if = "Option::is_none")]
    pub auto_create_forum_thread: Option<bool>,
    #[serde(rename = "isRecommendationBlocked", skip_serializing_if = "Option::is_none")]
    pub is_recommendation_blocked: Option<bool>,
    #[serde(rename = "isReviewBlocked", skip_serializing_if = "Option::is_none")]
    pub is_review_blocked: Option<bool>,
    #[serde(rename = "modNotes", skip_serializing_if = "Option::is_none")]
    pub mod_notes: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaCharacter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<CharacterRole>,
    #[serde(rename = "roleNotes", skip_serializing_if = "Option::is_none")]
    pub role_notes: Option<String>,
    #[serde(rename = "dubGroup", skip_serializing_if = "Option::is_none")]
    pub dub_group: Option<String>,
    #[serde(rename = "characterName", skip_serializing_if = "Option::is_none")]
    pub character_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character: Option<Character>,
    #[serde(rename = "voiceActor", skip_serializing_if = "Option::is_none")]
    pub voice_actor: Option<Staff>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    edges: Option<Vec<MediaEdge>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodes: Option<Vec<Media>>,
    #[serde(rename = "pageInfo", skip_serializing_if = "Option::is_none")]
    page_info: Option<PageInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaCoverImage {
    #[serde(rename = "extraLarge", skip_serializing_if = "Option::is_none")]
    pub extra_large: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaDataChangeNotification {
    pub id: i32,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub notification_type: Option<NotificationType>,
    #[serde(rename = "mediaId")]
    pub media_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaDeletionNotification {
    pub id: i32,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub notification_type: Option<NotificationType>,
    #[serde(rename = "deletedMediaTitle")]
    pub deleted_media_title: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaEdge {
    #[serde(skip_serializing_if = "Option::is_none")]
    node: Option<Media>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i32>,
    #[serde(rename = "relationType", skip_serializing_if = "Option::is_none")]
    relation_type: Option<MediaRelation>,
    #[serde(rename = "isMainStudio", skip_serializing_if = "Option::is_none")]
    pub is_main_studio: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characters: Option<Vec<Character>>,
    #[serde(rename = "characterRole", skip_serializing_if = "Option::is_none")]
    pub character_role: Option<CharacterRole>,
    #[serde(rename = "characterName", skip_serializing_if = "Option::is_none")]
    pub character_name: Option<String>,
    #[serde(rename = "roleNotes", skip_serializing_if = "Option::is_none")]
    pub role_notes: Option<String>,
    #[serde(rename = "dubGroup", skip_serializing_if = "Option::is_none")]
    pub dub_group: Option<String>,
    #[serde(rename = "staffRole", skip_serializing_if = "Option::is_none")]
    pub staff_role: Option<String>,
    #[serde(rename = "voiceActors", skip_serializing_if = "Option::is_none")]
    pub voice_actors: Option<Vec<Staff>>,
    #[serde(rename = "voiceActorRoles", skip_serializing_if = "Option::is_none")]
    pub voice_actor_roles: Option<Vec<StaffRoleType>>,
    #[serde(rename = "favouriteOrder", skip_serializing_if = "Option::is_none")]
    pub favourite_order: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaExternalLink {
    id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    site: String,
    #[serde(rename = "siteId", skip_serializing_if = "Option::is_none")]
    site_id: Option<i32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    link_type: Option<ExternalLinkType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "isDisabled", skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaMergeNotification {
    pub id: i32,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub notification_type: Option<NotificationType>,
    #[serde(rename = "mediaId")]
    pub media_id: i32,
    #[serde(rename = "deletedMediaTitles", skip_serializing_if = "Option::is_none")]
    pub deleted_media_titles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaRank {
    pub id: i32,
    pub rank: i32,
    #[serde(rename = "type")]
    pub rank_type: MediaRankType,
    pub format: MediaFormat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<MediaSeason>,
    #[serde(rename = "allTime", skip_serializing_if = "Option::is_none")]
    pub all_time: Option<bool>,
    #[serde(rename = "context")]
    pub context: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaStreamingEpisode {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaSubmission {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitter: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SubmissionStatus>,
    #[serde(rename = "submitterStats", skip_serializing_if = "Option::is_none")]
    pub submitter_stats: Option<Json>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission: Option<Media>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characters: Option<Vec<MediaSubmissionComparison>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staff: Option<Vec<MediaSubmissionComparison>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studios: Option<Vec<MediaSubmissionComparison>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relations: Option<Vec<MediaEdge>>,
    #[serde(rename = "externalLinks", skip_serializing_if = "Option::is_none")]
    pub external_links: Option<Vec<MediaSubmissionComparison>>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaSubmissionComparison {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission: Option<MediaSubmissionEdge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character: Option<MediaCharacter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staff: Option<StaffEdge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studio: Option<StudioEdge>,
    #[serde(rename = "externalLink", skip_serializing_if = "Option::is_none")]
    pub external_link: Option<MediaExternalLink>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaSubmissionEdge {
    pub id: i32,
    #[serde(rename = "characterRole", skip_serializing_if = "Option::is_none")]
    pub character_role: Option<CharacterRole>,
    #[serde(rename = "staffRole", skip_serializing_if = "Option::is_none")]
    pub staff_role: Option<String>,
    #[serde(rename = "roleNotes", skip_serializing_if = "Option::is_none")]
    pub role_notes: Option<String>,
    #[serde(rename = "dubGroup", skip_serializing_if = "Option::is_none")]
    pub dub_group: Option<String>,
    #[serde(rename = "characterName", skip_serializing_if = "Option::is_none")]
    pub character_name: Option<String>,
    #[serde(rename = "isMain", skip_serializing_if = "Option::is_none")]
    pub is_main: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character: Option<Character>,
    #[serde(rename = "characterSubmission", skip_serializing_if = "Option::is_none")]
    pub character_submission: Option<Character>,
    #[serde(rename = "voiceActor", skip_serializing_if = "Option::is_none")]
    pub voice_actor: Option<Staff>,
    #[serde(rename = "voiceActorSubmission", skip_serializing_if = "Option::is_none")]
    pub voice_actor_submission: Option<Staff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staff: Option<Staff>,
    #[serde(rename = "staffSubmission", skip_serializing_if = "Option::is_none")]
    pub staff_submission: Option<Staff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studio: Option<Studio>,
    #[serde(rename = "externalLink", skip_serializing_if = "Option::is_none")]
    pub external_link: Option<MediaExternalLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaTag {
    pub id: i32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    #[serde(rename = "isGeneralSpoiler", skip_serializing_if = "Option::is_none")]
    pub is_general_spoiler: Option<bool>,
    #[serde(rename = "isMediaSpoiler", skip_serializing_if = "Option::is_none")]
    pub is_media_spoiler: Option<bool>,
    #[serde(rename = "isAdult", skip_serializing_if = "Option::is_none")]
    pub is_adult: Option<bool>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaTitle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub romaji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub english: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native: Option<String>,
    #[serde(rename = "userPreferred", skip_serializing_if = "Option::is_none")]
    pub user_preferred: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaTrailer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaTrend {
    #[serde(rename = "mediaId")]
    pub media_id: i32,
    pub date: i32,
    pub trending: i32,
    #[serde(rename = "averageScore", skip_serializing_if = "Option::is_none")]
    pub average_score: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub popularity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_progress: Option<i32>,
    pub releasing: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaTrendConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<MediaTrendEdge>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<MediaTrend>>,
    #[serde(rename = "pageInfo", skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaTrendEdge {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<MediaTrend>,
}
