use crate::objects::activity::{ActivityReply, MessageActivity};
use crate::objects::airing::AiringSchedule;
use crate::objects::character::Character;
use crate::objects::common::PageInfo;
use crate::objects::favourites::Favourites;
use crate::objects::media::Media;
use crate::objects::media_list::MediaList;
use crate::objects::recommendation::Recommendation;
use crate::objects::review::Review;
use crate::objects::staff::Staff;
use crate::objects::studio::Studio;
use crate::objects::user::User;
use crate::unions::activity::ActivityUnion;
use crate::unions::likeable::LikeableUnion;
use crate::unions::notification::NotificationUnion;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Top-level GraphQL response wrapper
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQLResponse<T> {
    pub data: T,
}

/// Generic response wrapper for paginated data
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResponse<T> {
    #[serde(rename = "Page")]
    pub page: Page<T>,
}

/// Generic page structure
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub page_info: PageInfo,
    #[serde(flatten)]
    pub data: T,
}

/// Generic viewer response wrapper
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewerResponse<T> {
    #[serde(rename = "Viewer")]
    pub viewer: T,
}

/// Specific response data types
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationData {
    pub notifications: Vec<NotificationUnion>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaData {
    pub media: Vec<Media>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaListData {
    pub media_list: Vec<MediaList>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterData {
    pub characters: Vec<Character>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffData {
    pub staff: Vec<Staff>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioData {
    pub studios: Vec<Studio>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub users: Vec<User>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityData {
    pub activities: Vec<ActivityUnion>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityReplyData {
    pub activity_replies: Vec<ActivityReply>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiringData {
    pub airing_schedules: Vec<AiringSchedule>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiringScheduleResponse {
    #[serde(rename = "AiringSchedule")]
    pub airing_schedule: AiringSchedule,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationData {
    pub recommendations: Vec<Recommendation>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewData {
    pub reviews: Vec<Review>,
}

/// Single item response wrappers
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaResponse {
    #[serde(rename = "Media")]
    pub media: Media,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterResponse {
    #[serde(rename = "Character")]
    pub character: Character,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffResponse {
    #[serde(rename = "Staff")]
    pub staff: Staff,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioResponse {
    #[serde(rename = "Studio")]
    pub studio: Studio,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    #[serde(rename = "User")]
    pub user: User,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityResponse {
    #[serde(rename = "Activity")]
    pub activity: ActivityUnion,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewResponse {
    #[serde(rename = "Review")]
    pub review: Review,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationResponse {
    #[serde(rename = "Recommendation")]
    pub recommendation: Recommendation,
}

/// Viewer-specific response data types
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewerUnreadCount {
    pub id: i32,
    pub name: String,
    pub unread_notification_count: i32,
}

/// Type aliases for common response patterns
pub type NotificationResponse = GraphQLResponse<PageResponse<NotificationData>>;
pub type MediaListResponse = GraphQLResponse<PageResponse<MediaData>>;
pub type UserMediaListResponse = GraphQLResponse<PageResponse<MediaListData>>;
pub type CharacterListResponse = GraphQLResponse<PageResponse<CharacterData>>;
pub type StaffListResponse = GraphQLResponse<PageResponse<StaffData>>;
pub type StudioListResponse = GraphQLResponse<PageResponse<StudioData>>;
pub type UserListResponse = GraphQLResponse<PageResponse<UserData>>;
pub type ActivityListResponse = GraphQLResponse<PageResponse<ActivityData>>;
pub type ActivityReplyListResponse = GraphQLResponse<PageResponse<ActivityReplyData>>;
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

// Activity mutation response types
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveTextActivityData {
    #[serde(rename = "SaveTextActivity")]
    pub save_text_activity: ActivityUnion,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveMessageActivityData {
    #[serde(rename = "SaveMessageActivity")]
    pub save_message_activity: MessageActivity,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteActivityData {
    #[serde(rename = "DeleteActivity")]
    pub deleted: DeletedResponse,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletedResponse {
    pub deleted: bool,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveActivityReplyData {
    #[serde(rename = "SaveActivityReply")]
    pub save_activity_reply: ActivityReply,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteActivityReplyData {
    #[serde(rename = "DeleteActivityReply")]
    pub deleted: DeletedResponse,
}

pub type SaveTextActivityResponse = GraphQLResponse<SaveTextActivityData>;
pub type SaveMessageActivityResponse = GraphQLResponse<SaveMessageActivityData>;
pub type DeleteActivityResponse = GraphQLResponse<DeleteActivityData>;
pub type SaveActivityReplyResponse = GraphQLResponse<SaveActivityReplyData>;
pub type DeleteActivityReplyResponse = GraphQLResponse<DeleteActivityReplyData>;

// MediaList single item response types
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaListEntryData {
    #[serde(rename = "SaveMediaListEntry")]
    pub save_media_list_entry: MediaList,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteMediaListData {
    #[serde(rename = "DeleteMediaListEntry")]
    pub deleted: bool,
}

pub type SaveMediaListEntryResponse = GraphQLResponse<MediaListEntryData>;
pub type DeleteMediaListEntryResponse = GraphQLResponse<DeleteMediaListData>;

// Common endpoint response types
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleLikeData {
    #[serde(rename = "ToggleLikeV2")]
    pub toggle_like_v2: LikeableUnion,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleFollowData {
    #[serde(rename = "ToggleFollow")]
    pub toggle_follow: User,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleFavouriteData {
    #[serde(rename = "ToggleFavourite")]
    pub toggle_favourite: Favourites,
}

pub type ToggleLikeResponse = GraphQLResponse<ToggleLikeData>;
pub type ToggleFollowResponse = GraphQLResponse<ToggleFollowData>;
pub type ToggleFavouriteResponse = GraphQLResponse<ToggleFavouriteData>;
