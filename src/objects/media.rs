use serde::{Deserialize, Serialize};

use crate::{enums::{media::{MediaFormat, MediaSeason, MediaSource, MediaStatus, MediaType}, notification::NotificationType}, objects::common::{FuzzyDate, PageInfo}};

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
    pub media_list_entry: Option<MediaList>,
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
    pub mod_notes: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct  MediaTitle {
    pub romaji: Option<String>,
    pub english: Option<String>,
    pub native: Option<String>,
    #[serde(rename = "userPreferred")]
    pub user_preferred: Option<String>,
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
    page_info: Option<PageInfo>
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
    pub media: Option<Media>
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