pub mod anime;
pub mod character;
pub mod manga;
pub mod media_list;
pub mod social;
pub mod staff;
pub mod user;

// Re-export specific types to avoid ambiguity
pub use anime::{
    AiringSchedule, Anime, FuzzyDate, MediaCoverImage, MediaFormat, MediaSeason, MediaSource,
    MediaStatus, MediaTitle, MediaTrailer, Studio, StudioConnection, StudioEdge,
};
pub use character::{Character, CharacterImage, CharacterName};
pub use manga::Manga;
pub use media_list::{MediaList, MediaListMedia, MediaListStatus};
pub use social::{
    Activity, ActivityReply, ActivityType, AiringMedia, AiringSchedule as SocialAiringSchedule,
    ListActivity, MediaType, MessageActivity, Notification, NotificationMedia, NotificationType,
    NotificationUser, Recommendation, RecommendationMedia, RecommendationRating,
    RecommendationUser, Review, ReviewMedia, ReviewRating, ReviewUser, Studio as SocialStudio,
    TextActivity, Thread, ThreadCategory, ThreadComment, ThreadUser,
};
pub use staff::{Staff, StaffImage, StaffName};
pub use user::{
    Favourites, MediaListOptions, MediaListTypeOptions, NotificationOption, User, UserAvatar,
    UserOptions, UserStatistics, UserStatisticsType,
};
