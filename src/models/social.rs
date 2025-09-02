use super::MediaCoverImage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Studio {
    pub id: i32,
    pub name: String,
    #[serde(rename = "isAnimationStudio")]
    pub is_animation_studio: bool,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
    pub favourites: Option<i32>,
    #[serde(rename = "isFavourite")]
    pub is_favourite: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thread {
    pub id: i32,
    pub title: String,
    pub body: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "replyUserId")]
    pub reply_user_id: Option<i32>,
    #[serde(rename = "replyCommentId")]
    pub reply_comment_id: Option<i32>,
    #[serde(rename = "categories")]
    pub categories: Option<Vec<ThreadCategory>>,
    #[serde(rename = "mediaCategories")]
    pub media_categories: Option<Vec<ThreadCategory>>,
    #[serde(rename = "isLocked")]
    pub is_locked: Option<bool>,
    #[serde(rename = "isSticky")]
    pub is_sticky: Option<bool>,
    #[serde(rename = "isSubscribed")]
    pub is_subscribed: Option<bool>,
    #[serde(rename = "likeCount")]
    pub like_count: i32,
    #[serde(rename = "isLiked")]
    pub is_liked: Option<bool>,
    #[serde(rename = "repliedAt")]
    pub replied_at: Option<i32>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    #[serde(rename = "updatedAt")]
    pub updated_at: i32,
    pub user: Option<ThreadUser>,
    #[serde(rename = "replyUser")]
    pub reply_user: Option<ThreadUser>,
    #[serde(rename = "replyCount")]
    pub reply_count: Option<i32>,
    #[serde(rename = "viewCount")]
    pub view_count: Option<i32>,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadCategory {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadUser {
    pub id: i32,
    pub name: String,
    pub avatar: Option<UserAvatar>,
    #[serde(rename = "donatorTier")]
    pub donator_tier: Option<i32>,
    #[serde(rename = "donatorBadge")]
    pub donator_badge: Option<String>,
    #[serde(rename = "moderatorRoles")]
    pub moderator_roles: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAvatar {
    pub large: Option<String>,
    pub medium: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadComment {
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "threadId")]
    pub thread_id: i32,
    pub comment: String,
    #[serde(rename = "likeCount")]
    pub like_count: i32,
    #[serde(rename = "isLiked")]
    pub is_liked: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    #[serde(rename = "updatedAt")]
    pub updated_at: i32,
    pub user: Option<ThreadUser>,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "mediaId")]
    pub media_id: i32,
    #[serde(rename = "mediaType")]
    pub media_type: Option<MediaType>,
    pub summary: Option<String>,
    pub body: String,
    pub rating: Option<i32>,
    #[serde(rename = "ratingAmount")]
    pub rating_amount: Option<i32>,
    #[serde(rename = "userRating")]
    pub user_rating: Option<ReviewRating>,
    pub score: Option<i32>,
    #[serde(rename = "private")]
    pub is_private: Option<bool>,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    #[serde(rename = "updatedAt")]
    pub updated_at: i32,
    pub user: Option<ReviewUser>,
    pub media: Option<ReviewMedia>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaType {
    ANIME,
    MANGA,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReviewRating {
    #[serde(rename = "NO_VOTE")]
    NoVote,
    #[serde(rename = "UP_VOTE")]
    UpVote,
    #[serde(rename = "DOWN_VOTE")]
    DownVote,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewUser {
    pub id: i32,
    pub name: String,
    pub avatar: Option<UserAvatar>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewMedia {
    pub id: i32,
    pub title: Option<MediaTitle>,
    #[serde(rename = "coverImage")]
    pub cover_image: Option<MediaCoverImage>,
    #[serde(rename = "bannerImage")]
    pub banner_image: Option<String>,
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
pub struct Recommendation {
    pub id: i32,
    pub rating: Option<i32>,
    #[serde(rename = "userRating")]
    pub user_rating: Option<RecommendationRating>,
    pub media: Option<RecommendationMedia>,
    #[serde(rename = "mediaRecommendation")]
    pub media_recommendation: Option<RecommendationMedia>,
    pub user: Option<RecommendationUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationRating {
    #[serde(rename = "NO_RATING")]
    NoRating,
    #[serde(rename = "RATE_UP")]
    RateUp,
    #[serde(rename = "RATE_DOWN")]
    RateDown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationMedia {
    pub id: i32,
    pub title: Option<MediaTitle>,
    #[serde(rename = "coverImage")]
    pub cover_image: Option<MediaCoverImage>,
    pub format: Option<MediaFormat>,
    #[serde(rename = "averageScore")]
    pub average_score: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaFormat {
    TV,
    #[serde(rename = "TV_SHORT")]
    TvShort,
    MOVIE,
    SPECIAL,
    OVA,
    ONA,
    MUSIC,
    MANGA,
    NOVEL,
    #[serde(rename = "ONE_SHOT")]
    OneShot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationUser {
    pub id: i32,
    pub name: String,
    pub avatar: Option<UserAvatar>,
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
    pub media: Option<AiringMedia>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiringMedia {
    pub id: i32,
    pub title: Option<MediaTitle>,
    #[serde(rename = "coverImage")]
    pub cover_image: Option<MediaCoverImage>,
    #[serde(rename = "bannerImage")]
    pub banner_image: Option<String>,
    pub episodes: Option<i32>,
    pub format: Option<MediaFormat>,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: Option<ActivityType>,
    #[serde(rename = "replyCount")]
    pub reply_count: i32,
    #[serde(rename = "likeCount")]
    pub like_count: i32,
    #[serde(rename = "isLiked")]
    pub is_liked: Option<bool>,
    #[serde(rename = "isSubscribed")]
    pub is_subscribed: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    pub user: Option<ActivityUser>,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityType {
    Text,
    AnimeList,
    MangaList,
    Message,
    MediaList,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityUser {
    pub id: i32,
    pub name: String,
    pub avatar: Option<UserAvatar>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextActivity {
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
    pub text: Option<String>,
    #[serde(rename = "replyCount")]
    pub reply_count: i32,
    #[serde(rename = "likeCount")]
    pub like_count: i32,
    #[serde(rename = "isLiked")]
    pub is_liked: Option<bool>,
    #[serde(rename = "isPinned")]
    pub is_pinned: Option<bool>,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    pub user: Option<ActivityUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListActivity {
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: Option<ActivityType>,
    pub status: Option<String>,
    pub progress: Option<String>,
    #[serde(rename = "replyCount")]
    pub reply_count: i32,
    #[serde(rename = "likeCount")]
    pub like_count: i32,
    #[serde(rename = "isLiked")]
    pub is_liked: Option<bool>,
    #[serde(rename = "isPinned")]
    pub is_pinned: Option<bool>,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    pub user: Option<ActivityUser>,
    pub media: Option<ActivityMedia>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityMedia {
    pub id: i32,
    #[serde(rename = "type")]
    pub media_type: Option<MediaType>,
    pub title: Option<MediaTitle>,
    #[serde(rename = "coverImage")]
    pub cover_image: Option<MediaCoverImage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageActivity {
    pub id: i32,
    #[serde(rename = "recipientId")]
    pub recipient_id: Option<i32>,
    #[serde(rename = "messengerId")]
    pub messenger_id: Option<i32>,
    #[serde(rename = "type")]
    pub activity_type: Option<ActivityType>,
    #[serde(rename = "replyCount")]
    pub reply_count: i32,
    pub message: Option<String>,
    #[serde(rename = "isLocked")]
    pub is_locked: Option<bool>,
    #[serde(rename = "isSubscribed")]
    pub is_subscribed: Option<bool>,
    #[serde(rename = "likeCount")]
    pub like_count: i32,
    #[serde(rename = "isLiked")]
    pub is_liked: Option<bool>,
    #[serde(rename = "isPrivate")]
    pub is_private: Option<bool>,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    pub recipient: Option<ActivityUser>,
    pub messenger: Option<ActivityUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityReply {
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
    #[serde(rename = "activityId")]
    pub activity_id: Option<i32>,
    pub text: Option<String>,
    #[serde(rename = "likeCount")]
    pub like_count: i32,
    #[serde(rename = "isLiked")]
    pub is_liked: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    pub user: Option<ActivityUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
    #[serde(rename = "type")]
    pub notification_type: Option<NotificationType>,
    #[serde(rename = "animeId")]
    pub anime_id: Option<i32>,
    pub episode: Option<i32>,
    pub contexts: Option<Vec<String>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<i32>,
    pub media: Option<NotificationMedia>,
    pub user: Option<NotificationUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationType {
    ActivityMessage,
    ActivityReply,
    Following,
    ActivityMention,
    ThreadCommentMention,
    ThreadSubscribed,
    ThreadCommentReply,
    Airing,
    ActivityLike,
    ActivityReplyLike,
    ThreadLike,
    ThreadCommentLike,
    ActivityReplySubscribed,
    RelatedMediaAddition,
    MediaDataChange,
    MediaMerge,
    MediaDeletion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationMedia {
    pub id: i32,
    #[serde(rename = "type")]
    pub media_type: Option<MediaType>,
    pub title: Option<MediaTitle>,
    #[serde(rename = "coverImage")]
    pub cover_image: Option<MediaCoverImage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationUser {
    pub id: i32,
    pub name: String,
    pub avatar: Option<UserAvatar>,
}
