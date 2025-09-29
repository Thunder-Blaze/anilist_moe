use serde::{Deserialize, Serialize};
use serde_json::Value as Json;

use crate::{
    enums::{
        mods::ModRole,
        user::{UserStaffNameLanguage, UserTitleLanguage},
    },
    objects::{
        activity::ListActivityOption, favourites::Favourites, media_list::MediaListOptions,
        notification::NotificationOption, stats::UserStatisticTypes,
    },
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<UserAvatar>,
    #[serde(rename = "bannerImage", skip_serializing_if = "Option::is_none")]
    pub banner_image: Option<String>,
    #[serde(rename = "isFollowing", skip_serializing_if = "Option::is_none")]
    pub is_following: Option<bool>,
    #[serde(rename = "isFollower", skip_serializing_if = "Option::is_none")]
    pub is_follower: Option<bool>,
    #[serde(rename = "isBlocked", skip_serializing_if = "Option::is_none")]
    pub is_blocked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bans: Option<Json>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<UserOptions>,
    #[serde(rename = "mediaListOptions", skip_serializing_if = "Option::is_none")]
    pub media_list_options: Option<MediaListOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favourites: Option<Favourites>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<UserStatisticTypes>,
    #[serde(rename = "unreadNotificationCount", skip_serializing_if = "Option::is_none")]
    pub unread_notification_count: Option<i32>,
    #[serde(rename = "siteUrl", skip_serializing_if = "Option::is_none")]
    pub site_url: Option<String>,
    #[serde(rename = "donatorTier", skip_serializing_if = "Option::is_none")]
    pub donator_tier: Option<i32>,
    #[serde(rename = "donatorBadge", skip_serializing_if = "Option::is_none")]
    pub donator_badge: Option<String>,
    #[serde(rename = "moderatorRoles", skip_serializing_if = "Option::is_none")]
    pub moderator_roles: Option<Vec<ModRole>>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i32>,
    #[serde(rename = "previousNames", skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserModData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alts: Option<Vec<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bans: Option<Json>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<Json>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts: Option<Json>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserOptions {
    #[serde(rename = "titleLanguage", skip_serializing_if = "Option::is_none")]
    pub title_language: Option<UserTitleLanguage>,
    #[serde(rename = "displayAdultContent", skip_serializing_if = "Option::is_none")]
    pub display_adult_content: Option<bool>,
    #[serde(rename = "airingNotifications", skip_serializing_if = "Option::is_none")]
    pub airing_notifications: Option<bool>,
    #[serde(rename = "profileColor", skip_serializing_if = "Option::is_none")]
    pub profile_color: Option<String>,
    #[serde(rename = "notificationOptions", skip_serializing_if = "Option::is_none")]
    pub notification_options: Option<Vec<NotificationOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "activityMergeTime", skip_serializing_if = "Option::is_none")]
    pub activity_merge_time: Option<i32>,
    #[serde(rename = "staffNameLanguage", skip_serializing_if = "Option::is_none")]
    pub staff_name_language: Option<UserStaffNameLanguage>,
    #[serde(rename = "restrictMessagesToFollowing", skip_serializing_if = "Option::is_none")]
    pub restrict_messages_to_following: Option<bool>,
    #[serde(rename = "disabledListActivity", skip_serializing_if = "Option::is_none")]
    pub disabled_list_activity: Option<Vec<ListActivityOption>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPreviousName {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i32>,
}
