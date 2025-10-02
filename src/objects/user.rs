use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
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

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub about: Option<String>,
    pub avatar: Option<UserAvatar>,
    pub banner_image: Option<String>,
    pub is_following: Option<bool>,
    pub is_follower: Option<bool>,
    pub is_blocked: Option<bool>,
    pub bans: Option<Json>,
    pub options: Option<UserOptions>,
    pub media_list_options: Option<MediaListOptions>,
    pub favourites: Option<Favourites>,
    pub statistics: Option<UserStatisticTypes>,
    pub unread_notification_count: Option<i32>,
    pub site_url: Option<String>,
    pub donator_tier: Option<i32>,
    pub donator_badge: Option<String>,
    pub moderator_roles: Option<Vec<ModRole>>,
    pub created_at: Option<i32>,
    pub updated_at: Option<i32>,
    pub previous_names: Option<Vec<UserPreviousName>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserActivityHistory {
    pub date: Option<i32>,
    pub amount: Option<i32>,
    pub level: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAvatar {
    pub large: Option<String>,
    pub medium: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserModData {
    pub alts: Option<Vec<User>>,
    pub bans: Option<Json>,
    pub ip: Option<Json>,
    pub counts: Option<Json>,
    pub privacy: Option<i32>,
    pub email: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserOptions {
    pub title_language: Option<UserTitleLanguage>,
    pub display_adult_content: Option<bool>,
    pub airing_notifications: Option<bool>,
    pub profile_color: Option<String>,
    pub notification_options: Option<Vec<NotificationOption>>,
    pub timezone: Option<String>,
    pub activity_merge_time: Option<i32>,
    pub staff_name_language: Option<UserStaffNameLanguage>,
    pub restrict_messages_to_following: Option<bool>,
    pub disabled_list_activity: Option<Vec<ListActivityOption>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPreviousName {
    pub name: Option<String>,
    pub created_at: Option<i32>,
    pub updated_at: Option<i32>,
}
