use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    enums::{
        media::{CountryCode, MediaFormat},
        media_list::MediaListStatus,
    },
    objects::{
        common::PageInfo, media::MediaTag, staff::Staff, studio::Studio, user::UserActivityHistory,
    },
};

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatStats {
    pub format: Option<MediaFormat>,
    pub amount: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenreStats {
    pub genre: Option<String>,
    pub amount: Option<i32>,
    pub mean_score: Option<i32>,
    pub time_watched: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListScoreStats {
    pub mean_score: Option<i32>,
    pub standard_deviation: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaStats {
    pub score_distribution: Option<Vec<ScoreDistribution>>,
    pub status_distribution: Option<Vec<StatusDistribution>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreDistribution {
    pub score: Option<i32>,
    pub amount: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusDistribution {
    pub status: Option<MediaListStatus>,
    pub amount: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteStatistics {
    pub users: Option<SiteTrendConnection>,
    pub anime: Option<SiteTrendConnection>,
    pub manga: Option<SiteTrendConnection>,
    pub characters: Option<SiteTrendConnection>,
    pub staff: Option<SiteTrendConnection>,
    pub studios: Option<SiteTrendConnection>,
    pub reviews: Option<SiteTrendConnection>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteTrend {
    pub date: i32,
    pub count: i32,
    pub change: i32,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteTrendConnection {
    pub edges: Option<Vec<SiteTrendEdge>>,
    pub nodes: Option<Vec<SiteTrend>>,
    pub page_info: Option<PageInfo>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteTrendEdge {
    pub node: Option<SiteTrend>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffStats {
    pub staff: Option<Staff>,
    pub amount: Option<i32>,
    pub mean_score: Option<i32>,
    pub time_watched: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioStats {
    pub studio: Option<Studio>,
    pub amount: Option<i32>,
    pub mean_score: Option<i32>,
    pub time_watched: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagStats {
    pub tag: Option<MediaTag>,
    pub amount: Option<i32>,
    pub mean_score: Option<i32>,
    pub time_watched: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserCountryStatistic {
    pub count: Option<i32>,
    pub mean_score: Option<f32>,
    pub minutes_watched: Option<i32>,
    pub chapters_read: Option<i32>,
    pub media_ids: Option<Vec<i32>>,
    pub country: Option<CountryCode>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserFormatStatistic {
    pub count: Option<i32>,
    pub mean_score: Option<f32>,
    pub minutes_watched: Option<i32>,
    pub chapters_read: Option<i32>,
    pub media_ids: Option<Vec<i32>>,
    pub format: Option<MediaFormat>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGenreStatistic {
    pub count: Option<i32>,
    pub mean_score: Option<f32>,
    pub minutes_watched: Option<i32>,
    pub chapters_read: Option<i32>,
    pub media_ids: Option<Vec<i32>>,
    pub genre: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserLengthStatistic {
    pub count: Option<i32>,
    pub mean_score: Option<f32>,
    pub minutes_watched: Option<i32>,
    pub chapters_read: Option<i32>,
    pub media_ids: Option<Vec<i32>>,
    pub length: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserReleaseYearStatistic {
    pub count: Option<i32>,
    pub mean_score: Option<f32>,
    pub minutes_watched: Option<i32>,
    pub chapters_read: Option<i32>,
    pub media_ids: Option<Vec<i32>>,
    pub release_year: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserScoreStatistic {
    pub count: Option<i32>,
    pub mean_score: Option<f32>,
    pub minutes_watched: Option<i32>,
    pub chapters_read: Option<i32>,
    pub media_ids: Option<Vec<i32>>,
    pub score: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStaffStatistic {
    pub count: Option<i32>,
    pub mean_score: Option<f32>,
    pub minutes_watched: Option<i32>,
    pub chapters_read: Option<i32>,
    pub media_ids: Option<Vec<i32>>,
    pub staff: Option<Staff>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStartYearStatistic {
    pub count: Option<i32>,
    pub mean_score: Option<f32>,
    pub minutes_watched: Option<i32>,
    pub chapters_read: Option<i32>,
    pub media_ids: Option<Vec<i32>>,
    pub start_year: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStatisticTypes {
    pub anime: Option<UserStatistics>,
    pub manga: Option<UserStatistics>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStatistics {
    pub count: Option<i32>,
    pub mean_score: Option<f32>,
    pub standard_deviation: Option<f32>,
    pub minutes_watched: Option<i32>,
    pub episodes_watched: Option<i32>,
    pub chapters_read: Option<i32>,
    pub volumes_read: Option<i32>,
    pub formats: Option<Vec<UserFormatStatistic>>,
    pub statuses: Option<Vec<UserStatusStatistic>>,
    pub scores: Option<Vec<UserScoreStatistic>>,
    pub lengths: Option<Vec<UserLengthStatistic>>,
    pub release_years: Option<Vec<UserReleaseYearStatistic>>,
    pub start_years: Option<Vec<UserStartYearStatistic>>,
    pub genres: Option<Vec<UserGenreStatistic>>,
    pub tags: Option<Vec<UserTagStatistic>>,
    pub countries: Option<Vec<UserCountryStatistic>>,
    pub voice_actors: Option<Vec<UserVoiceActorStatistic>>,
    pub staff: Option<Vec<UserStaffStatistic>>,
    pub studios: Option<Vec<UserStudioStatistic>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStats {
    pub watched_time: Option<i32>,
    pub chapters_read: Option<i32>,
    pub activity_history: Option<Vec<UserActivityHistory>>,
    pub anime_status_distribution: Option<Vec<StatusDistribution>>,
    pub manga_status_distribution: Option<Vec<StatusDistribution>>,
    pub anime_score_distribution: Option<Vec<ScoreDistribution>>,
    pub manga_score_distribution: Option<Vec<ScoreDistribution>>,
    pub anime_list_scores: Option<ListScoreStats>,
    pub manga_list_scores: Option<ListScoreStats>,
    pub favoured_genres_overview: Option<Vec<GenreStats>>,
    pub favoured_genres: Option<Vec<GenreStats>>,
    pub favoured_tags: Option<Vec<TagStats>>,
    pub favoured_actors: Option<Vec<StaffStats>>,
    pub favoured_staff: Option<Vec<StaffStats>>,
    pub favoured_studios: Option<Vec<StudioStats>>,
    pub favoured_years: Option<Vec<YearStats>>,
    pub favoured_formats: Option<Vec<FormatStats>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStatusStatistic {
    pub count: Option<i32>,
    pub mean_score: Option<f32>,
    pub minutes_watched: Option<i32>,
    pub chapters_read: Option<i32>,
    pub media_ids: Option<Vec<i32>>,
    pub status: Option<MediaListStatus>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStudioStatistic {
    pub count: Option<i32>,
    pub mean_score: Option<f32>,
    pub minutes_watched: Option<i32>,
    pub chapters_read: Option<i32>,
    pub media_ids: Option<Vec<i32>>,
    pub studio: Option<Studio>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTagStatistic {
    pub count: Option<i32>,
    pub mean_score: Option<f32>,
    pub minutes_watched: Option<i32>,
    pub chapters_read: Option<i32>,
    pub media_ids: Option<Vec<i32>>,
    pub tag: Option<MediaTag>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserVoiceActorStatistic {
    pub count: Option<i32>,
    pub mean_score: Option<f32>,
    pub minutes_watched: Option<i32>,
    pub chapters_read: Option<i32>,
    pub media_ids: Option<Vec<i32>>,
    pub voice_actor: Option<Staff>,
    pub character_ids: Option<Vec<i32>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YearStats {
    pub year: Option<i32>,
    pub amount: Option<i32>,
    pub mean_score: Option<i32>,
}
