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
    #[serde(rename = "idMal")]
    pub id_mal: Option<i32>,
    pub title: Option<MediaTitle>,
    #[serde(rename = "type")]
    pub media_type: Option<MediaType>,
    pub format: Option<MediaFormat>,
    pub status: Option<MediaStatus>,
    pub description: Option<String>,
    #[serde(rename = "startDate")]
    pub start_date: Option<FuzzyDate>,
    #[serde(rename = "endDate")]
    pub end_date: Option<FuzzyDate>,
    pub season: Option<MediaSeason>,
    #[serde(rename = "seasonYear")]
    pub season_year: Option<i32>,
    #[serde(rename = "seasonInt")]
    pub season_int: Option<i32>,
    pub episodes: Option<i32>,
    pub duration: Option<i32>,
    pub chapters: Option<i32>,
    pub volumes: Option<i32>,
    #[serde(rename = "countryOfOrigin")]
    pub country_of_origin: Option<CountryCode>,
    #[serde(rename = "isLicensed")]
    pub is_licensed: Option<bool>,
    pub source: Option<MediaSource>,
    pub hashtag: Option<String>,
    pub trailer: Option<MediaTrailer>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i32>,
    #[serde(rename = "coverImage")]
    pub cover_image: Option<MediaCoverImage>,
    #[serde(rename = "bannerImage")]
    pub banner_image: Option<String>,
    pub genres: Option<Vec<String>>,
    pub synonyms: Option<Vec<String>>,
    #[serde(rename = "averageScore")]
    pub average_score: Option<i32>,
    #[serde(rename = "meanScore")]
    pub mean_score: Option<i32>,
    pub popularity: Option<i32>,
    #[serde(rename = "isLocked")]
    pub is_locked: Option<bool>,
    pub trending: Option<i32>,
    pub favourites: Option<i32>,
    pub tags: Option<Vec<MediaTag>>,
    pub relations: Option<MediaConnection>,
    pub characters: Option<CharacterConnection>,
    pub staff: Option<StaffConnection>,
    pub studios: Option<StudioConnection>,
    #[serde(rename = "isFavourite")]
    pub is_favourite: Option<bool>,
    #[serde(rename = "isFavouriteBlocked")]
    pub is_favourite_blocked: Option<bool>,
    #[serde(rename = "isAdult")]
    pub is_adult: Option<bool>,
    #[serde(rename = "nextAiringEpisode")]
    pub next_airing_episode: Option<AiringSchedule>,
    #[serde(rename = "airingSchedule")]
    pub airing_schedule: Option<AiringScheduleConnection>,
    pub trends: Option<MediaTrendConnection>,
    #[serde(rename = "externalLinks")]
    pub external_links: Option<Vec<MediaExternalLink>>,
    #[serde(rename = "streamingEpisodes")]
    pub streaming_episodes: Option<Vec<MediaStreamingEpisode>>,
    pub rankings: Option<Vec<MediaRank>>,
    #[serde(rename = "mediaListEntry")]
    pub media_list_entry: Option<Box<MediaList>>,
    pub reviews: Option<ReviewConnection>,
    pub recommendations: Option<RecommendationConnection>,
    pub stats: Option<MediaStats>,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
    #[serde(rename = "autoCreateForumThread")]
    pub auto_create_forum_thread: Option<bool>,
    #[serde(rename = "isRecommendationBlocked")]
    pub is_recommendation_blocked: Option<bool>,
    #[serde(rename = "isReviewBlocked")]
    pub is_review_blocked: Option<bool>,
    #[serde(rename = "modNotes")]
    pub mod_notes: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaCharacter {
    pub id: Option<i32>,
    pub role: Option<CharacterRole>,
    #[serde(rename = "roleNotes")]
    pub role_notes: Option<String>,
    #[serde(rename = "dubGroup")]
    pub dub_group: Option<String>,
    #[serde(rename = "characterName")]
    pub character_name: Option<String>,
    pub character: Option<Character>,
    #[serde(rename = "voiceActor")]
    pub voice_actor: Option<Staff>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaConnection {
    edges: Option<Vec<MediaEdge>>,
    nodes: Option<Vec<Media>>,
    #[serde(rename = "pageInfo")]
    page_info: Option<PageInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaCoverImage {
    #[serde(rename = "extraLarge")]
    pub extra_large: Option<String>,
    pub large: Option<String>,
    pub medium: Option<String>,
    pub color: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaDataChangeNotification {
    pub id: i32,
    #[serde(rename = "type")]
    pub notification_type: Option<NotificationType>,
    #[serde(rename = "mediaId")]
    pub media_id: i32,
    pub context: Option<String>,
    pub reason: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<i32>,
    pub media: Option<Media>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaDeletionNotification {
    pub id: i32,
    #[serde(rename = "type")]
    pub notification_type: Option<NotificationType>,
    #[serde(rename = "deletedMediaTitle")]
    pub deleted_media_title: i32,
    pub context: Option<String>,
    pub reason: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaEdge {
    node: Option<Media>,
    id: Option<i32>,
    #[serde(rename = "relationType")]
    relation_type: Option<MediaRelation>,
    #[serde(rename = "isMainStudio")]
    pub is_main_studio: Option<bool>,
    pub characters: Option<Vec<Character>>,
    #[serde(rename = "characterRole")]
    pub character_role: Option<CharacterRole>,
    #[serde(rename = "characterName")]
    pub character_name: Option<String>,
    #[serde(rename = "roleNotes")]
    pub role_notes: Option<String>,
    #[serde(rename = "dubGroup")]
    pub dub_group: Option<String>,
    #[serde(rename = "staffRole")]
    pub staff_role: Option<String>,
    #[serde(rename = "voiceActors")]
    pub voice_actors: Option<Vec<Staff>>,
    #[serde(rename = "voiceActorRoles")]
    pub voice_actor_roles: Option<Vec<StaffRoleType>>,
    #[serde(rename = "favouriteOrder")]
    pub favourite_order: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaExternalLink {
    id: i32,
    url: Option<String>,
    site: String,
    #[serde(rename = "siteId")]
    site_id: Option<i32>,
    #[serde(rename = "type")]
    link_type: Option<ExternalLinkType>,
    pub language: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub notes: Option<String>,
    #[serde(rename = "isDisabled")]
    pub is_disabled: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaMergeNotification {
    pub id: i32,
    #[serde(rename = "type")]
    pub notification_type: Option<NotificationType>,
    #[serde(rename = "mediaId")]
    pub media_id: i32,
    #[serde(rename = "deletedMediaTitles")]
    pub deleted_media_titles: Option<Vec<String>>,
    pub context: Option<String>,
    pub reason: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<i32>,
    pub media: Option<Media>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaRank {
    pub id: i32,
    pub rank: i32,
    #[serde(rename = "type")]
    pub rank_type: MediaRankType,
    pub format: MediaFormat,
    pub year: Option<i32>,
    pub season: Option<MediaSeason>,
    #[serde(rename = "allTime")]
    pub all_time: Option<bool>,
    #[serde(rename = "context")]
    pub context: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaStreamingEpisode {
    pub title: Option<String>,
    pub thumbnail: Option<String>,
    pub url: Option<String>,
    pub site: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaSubmission {
    pub id: i32,
    pub submitter: Option<User>,
    pub assignee: Option<User>,
    pub status: Option<SubmissionStatus>,
    #[serde(rename = "submitterStats")]
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
    #[serde(rename = "externalLinks")]
    pub external_links: Option<Vec<MediaSubmissionComparison>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaSubmissionComparison {
    pub submission: Option<MediaSubmissionEdge>,
    pub character: Option<MediaCharacter>,
    pub staff: Option<StaffEdge>,
    pub studio: Option<StudioEdge>,
    #[serde(rename = "externalLink")]
    pub external_link: Option<MediaExternalLink>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaSubmissionEdge {
    pub id: i32,
    #[serde(rename = "characterRole")]
    pub character_role: Option<CharacterRole>,
    #[serde(rename = "staffRole")]
    pub staff_role: Option<String>,
    #[serde(rename = "roleNotes")]
    pub role_notes: Option<String>,
    #[serde(rename = "dubGroup")]
    pub dub_group: Option<String>,
    #[serde(rename = "characterName")]
    pub character_name: Option<String>,
    #[serde(rename = "isMain")]
    pub is_main: Option<bool>,
    pub character: Option<Character>,
    #[serde(rename = "characterSubmission")]
    pub character_submission: Option<Character>,
    #[serde(rename = "voiceActor")]
    pub voice_actor: Option<Staff>,
    #[serde(rename = "voiceActorSubmission")]
    pub voice_actor_submission: Option<Staff>,
    pub staff: Option<Staff>,
    #[serde(rename = "staffSubmission")]
    pub staff_submission: Option<Staff>,
    pub studio: Option<Studio>,
    #[serde(rename = "externalLink")]
    pub external_link: Option<MediaExternalLink>,
    pub media: Option<Media>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaTag {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub rank: Option<i32>,
    #[serde(rename = "isGeneralSpoiler")]
    pub is_general_spoiler: Option<bool>,
    #[serde(rename = "isMediaSpoiler")]
    pub is_media_spoiler: Option<bool>,
    #[serde(rename = "isAdult")]
    pub is_adult: Option<bool>,
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaTitle {
    pub romaji: Option<String>,
    pub english: Option<String>,
    pub native: Option<String>,
    #[serde(rename = "userPreferred")]
    pub user_preferred: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaTrailer {
    pub id: Option<String>,
    pub site: Option<String>,
    pub thumbnail: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaTrend {
    #[serde(rename = "mediaId")]
    pub media_id: i32,
    pub date: i32,
    pub trending: i32,
    #[serde(rename = "averageScore")]
    pub average_score: Option<i32>,
    pub popularity: Option<i32>,
    pub in_progress: Option<i32>,
    pub releasing: bool,
    pub episode: Option<i32>,
    pub media: Option<Media>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaTrendConnection {
    pub edges: Option<Vec<MediaTrendEdge>>,
    pub nodes: Option<Vec<MediaTrend>>,
    #[serde(rename = "pageInfo")]
    pub page_info: Option<PageInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaTrendEdge {
    pub node: Option<MediaTrend>,
}
