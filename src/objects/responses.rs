use crate::objects::common::PageInfo;
use crate::objects::user::User;
use crate::objects::media::Media;
use crate::objects::character::Character;
use crate::objects::staff::Staff;
use crate::objects::studio::Studio;
use crate::unions::activity::ActivityUnion;
use crate::objects::airing::AiringSchedule;
use crate::objects::recommendation::Recommendation;
use crate::objects::review::Review;
use crate::unions::notification::NotificationUnion;
use serde::{Deserialize, Serialize};

/// Top-level GraphQL response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: T,
}

/// Generic response wrapper for paginated data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    #[serde(rename = "Page")]
    pub page: Page<T>,
}

/// Generic page structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page<T> {
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
    #[serde(flatten)]
    pub data: T,
}

/// Generic viewer response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewerResponse<T> {
    #[serde(rename = "Viewer")]
    pub viewer: T,
}

/// Specific response data types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationData {
    pub notifications: Vec<NotificationUnion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaData {
    pub media: Vec<Media>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterData {
    pub characters: Vec<Character>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffData {
    pub staff: Vec<Staff>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudioData {
    pub studios: Vec<Studio>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    pub users: Vec<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityData {
    pub activities: Vec<ActivityUnion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiringData {
    #[serde(rename = "airingSchedules")]
    pub airing_schedules: Vec<AiringSchedule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiringScheduleResponse {
    #[serde(rename = "AiringSchedule")]
    pub airing_schedule: AiringSchedule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationData {
    pub recommendations: Vec<Recommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewData {
    pub reviews: Vec<Review>,
}

/// Single item response wrappers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaResponse {
    #[serde(rename = "Media")]
    pub media: Media,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterResponse {
    #[serde(rename = "Character")]
    pub character: Character,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffResponse {
    #[serde(rename = "Staff")]
    pub staff: Staff,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudioResponse {
    #[serde(rename = "Studio")]
    pub studio: Studio,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    #[serde(rename = "User")]
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityResponse {
    #[serde(rename = "Activity")]
    pub activity: ActivityUnion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewResponse {
    #[serde(rename = "Review")]
    pub review: Review,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationResponse {
    #[serde(rename = "Recommendation")]
    pub recommendation: Recommendation,
}

/// Viewer-specific response data types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewerUnreadCount {
    pub id: i32,
    pub name: String,
    #[serde(rename = "unreadNotificationCount")]
    pub unread_notification_count: i32,
}

/// Type aliases for common response patterns
pub type NotificationResponse = GraphQLResponse<PageResponse<NotificationData>>;
pub type MediaListResponse = GraphQLResponse<PageResponse<MediaData>>;
pub type CharacterListResponse = GraphQLResponse<PageResponse<CharacterData>>;
pub type StaffListResponse = GraphQLResponse<PageResponse<StaffData>>;
pub type StudioListResponse = GraphQLResponse<PageResponse<StudioData>>;
pub type UserListResponse = GraphQLResponse<PageResponse<UserData>>;
pub type ActivityListResponse = GraphQLResponse<PageResponse<ActivityData>>;
pub type AiringListResponse = GraphQLResponse<PageResponse<AiringData>>;
pub type AiringSingleResponse = GraphQLResponse<AiringScheduleResponse>;
pub type RecommendationListResponse = GraphQLResponse<PageResponse<RecommendationData>>;
pub type ReviewListResponse = GraphQLResponse<PageResponse<ReviewData>>;
pub type UnreadCountResponse = GraphQLResponse<ViewerResponse<ViewerUnreadCount>>;

// Single item response types
pub type MediaSingleResponse = GraphQLResponse<MediaResponse>;
pub type CharacterSingleResponse = GraphQLResponse<CharacterResponse>;
pub type StaffSingleResponse = GraphQLResponse<StaffResponse>;
pub type StudioSingleResponse = GraphQLResponse<StudioResponse>;
pub type UserSingleResponse = GraphQLResponse<UserResponse>;
pub type ActivitySingleResponse = GraphQLResponse<ActivityResponse>;
pub type ReviewSingleResponse = GraphQLResponse<ReviewResponse>;
pub type RecommendationSingleResponse = GraphQLResponse<RecommendationResponse>;
