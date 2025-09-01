use serde::{Deserialize, Serialize};
use serde_json::{Value as Json};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub about: Option<String>,
    pub avatar: Option<UserAvatar>,
    #[serde(rename = "bannerImage")]
    pub banner_image: Option<String>,
    #[serde(rename = "isFollowing")]
    pub is_following: Option<bool>,
    #[serde(rename = "isFollower")]
    pub is_follower: Option<bool>,
    #[serde(rename = "isBlocked")]
    pub is_blocked: Option<bool>,
    pub bans: Option<Json>,
    pub options: Option<UserOptions>,
    #[serde(rename = "mediaListOptions")]
    pub media_list_options: Option<MediaListOptions>,
    pub favourites: Option<Favourites>,
    pub statistics: Option<UserStatisticTypes>,
    #[serde(rename = "unreadNotificationCount")]
    pub unread_notification_count: Option<i32>,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
    #[serde(rename = "donatorTier")]
    pub donator_tier: Option<i32>,
    #[serde(rename = "donatorBadge")]
    pub donator_badge: Option<String>,
    #[serde(rename = "moderatorRoles")]
    pub moderator_roles: Option<Vec<ModRole>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<i32>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i32>,
    #[serde(rename = "previousNames")]
    pub previous_names: Option<Vec<UserPreviousName>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserActivityHistory {
    pub date: i32,
    pub amount: i32,
    pub level: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserAvatar {
    pub large: Option<String>,
    pub medium: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserModData {
    pub alts: Option<Vec<User>>,
    pub bans: Option<Json>,
    pub ip: Option<Json>,
    pub counts: Option<Json>,
    pub privacy: Option<i32>,
    pub email: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserOptions {
    #[serde(rename = "titleLanguage")]
    pub title_language: Option<UserTitleLanguage>,
    #[serde(rename = "displayAdultContent")]
    pub display_adult_content: Option<bool>,
    #[serde(rename = "airingNotifications")]
    pub airing_notifications: Option<bool>,
    #[serde(rename = "profileColor")]
    pub profile_color: Option<String>,
    #[serde(rename = "notificationOptions")]
    pub notification_options: Option<Vec<NotificationOption>>,
    pub timezone: Option<String>,
    #[serde(rename = "activityMergeTime")]
    pub activity_merge_time: Option<i32>,
    #[serde(rename = "staffNameLanguage")]
    pub staff_name_language: Option<UserStaffNameLanguage>,
    #[serde(rename = "restrictMessagesToFollowing")]
    pub restrict_messages_to_following: Option<bool>,
    #[serde(rename = "disabledListActivity")]
    pub disabled_list_activity: Option<Vec<ListActivityOption>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPreviousName {
    pub name: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<i32>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i32>,
}