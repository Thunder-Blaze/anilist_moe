use serde::{Deserialize, Serialize};

use crate::{
    enums::{
        media::{CountryCode, MediaFormat},
        media_list::MediaListStatus,
    },
    objects::{
        common::PageInfo, media::MediaTag, staff::Staff, studio::Studio, user::UserActivityHistory,
    },
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FormatStats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<MediaFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenreStats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    #[serde(rename = "meanScore", skip_serializing_if = "Option::is_none")]
    pub mean_score: Option<i32>,
    #[serde(rename = "timeWatched", skip_serializing_if = "Option::is_none")]
    pub time_watched: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListScoreStats {
    #[serde(rename = "meanScore", skip_serializing_if = "Option::is_none")]
    pub mean_score: Option<i32>,
    #[serde(rename = "standardDeviation", skip_serializing_if = "Option::is_none")]
    pub standard_deviation: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaStats {
    #[serde(rename = "scoreDistribution", skip_serializing_if = "Option::is_none")]
    pub score_distribution: Option<Vec<ScoreDistribution>>,
    #[serde(rename = "statusDistribution", skip_serializing_if = "Option::is_none")]
    pub status_distribution: Option<Vec<StatusDistribution>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScoreDistribution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusDistribution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MediaListStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SiteStatistics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<SiteTrendConnection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anime: Option<SiteTrendConnection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manga: Option<SiteTrendConnection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characters: Option<SiteTrendConnection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staff: Option<SiteTrendConnection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studios: Option<SiteTrendConnection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviews: Option<SiteTrendConnection>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SiteTrend {
    pub date: i32,
    pub count: i32,
    pub change: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SiteTrendConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<SiteTrendEdge>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<SiteTrend>>,
    #[serde(rename = "pageInfo", skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SiteTrendEdge {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<SiteTrend>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StaffStats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staff: Option<Staff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    #[serde(rename = "meanScore", skip_serializing_if = "Option::is_none")]
    pub mean_score: Option<i32>,
    #[serde(rename = "timeWatched", skip_serializing_if = "Option::is_none")]
    pub time_watched: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StudioStats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studio: Option<Studio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    #[serde(rename = "meanScore", skip_serializing_if = "Option::is_none")]
    pub mean_score: Option<i32>,
    #[serde(rename = "timeWatched", skip_serializing_if = "Option::is_none")]
    pub time_watched: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagStats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<MediaTag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    #[serde(rename = "meanScore", skip_serializing_if = "Option::is_none")]
    pub mean_score: Option<i32>,
    #[serde(rename = "timeWatched", skip_serializing_if = "Option::is_none")]
    pub time_watched: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserCountryStatistic {
    pub count: i32,
    #[serde(rename = "meanScore")]
    pub mean_score: f32,
    #[serde(rename = "minutesWatched")]
    pub minutes_watched: i32,
    #[serde(rename = "chaptersRead")]
    pub chapters_read: i32,
    #[serde(rename = "mediaIds")]
    pub media_ids: Vec<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<CountryCode>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserFormatStatistic {
    pub count: i32,
    #[serde(rename = "meanScore")]
    pub mean_score: f32,
    #[serde(rename = "minutesWatched")]
    pub minutes_watched: i32,
    #[serde(rename = "chaptersRead")]
    pub chapters_read: i32,
    #[serde(rename = "mediaIds")]
    pub media_ids: Vec<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<MediaFormat>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserGenreStatistic {
    pub count: i32,
    #[serde(rename = "meanScore")]
    pub mean_score: f32,
    #[serde(rename = "minutesWatched")]
    pub minutes_watched: i32,
    #[serde(rename = "chaptersRead")]
    pub chapters_read: i32,
    #[serde(rename = "mediaIds")]
    pub media_ids: Vec<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserLengthStatistic {
    pub count: i32,
    #[serde(rename = "meanScore")]
    pub mean_score: f32,
    #[serde(rename = "minutesWatched")]
    pub minutes_watched: i32,
    #[serde(rename = "chaptersRead")]
    pub chapters_read: i32,
    #[serde(rename = "mediaIds")]
    pub media_ids: Vec<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserReleaseYearStatistic {
    pub count: i32,
    #[serde(rename = "meanScore")]
    pub mean_score: f32,
    #[serde(rename = "minutesWatched")]
    pub minutes_watched: i32,
    #[serde(rename = "chaptersRead")]
    pub chapters_read: i32,
    #[serde(rename = "mediaIds")]
    pub media_ids: Vec<i32>,
    #[serde(rename = "releaseYear", skip_serializing_if = "Option::is_none")]
    pub release_year: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserScoreStatistic {
    pub count: i32,
    #[serde(rename = "meanScore")]
    pub mean_score: f32,
    #[serde(rename = "minutesWatched")]
    pub minutes_watched: i32,
    #[serde(rename = "chaptersRead")]
    pub chapters_read: i32,
    #[serde(rename = "mediaIds")]
    pub media_ids: Vec<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserStaffStatistic {
    pub count: i32,
    #[serde(rename = "meanScore")]
    pub mean_score: f32,
    #[serde(rename = "minutesWatched")]
    pub minutes_watched: i32,
    #[serde(rename = "chaptersRead")]
    pub chapters_read: i32,
    #[serde(rename = "mediaIds")]
    pub media_ids: Vec<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staff: Option<Staff>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserStartYearStatistic {
    pub count: i32,
    #[serde(rename = "meanScore")]
    pub mean_score: f32,
    #[serde(rename = "minutesWatched")]
    pub minutes_watched: i32,
    #[serde(rename = "chaptersRead")]
    pub chapters_read: i32,
    #[serde(rename = "mediaIds")]
    pub media_ids: Vec<i32>,
    #[serde(rename = "startYear", skip_serializing_if = "Option::is_none")]
    pub start_year: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserStatisticTypes {
    #[serde(rename = "animeStatus", skip_serializing_if = "Option::is_none")]
    pub anime_status: Option<UserStatistics>,
    #[serde(rename = "mangaStatus", skip_serializing_if = "Option::is_none")]
    pub manga_status: Option<UserStatistics>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserStatistics {
    pub count: i32,
    #[serde(rename = "meanScore")]
    pub mean_score: f32,
    #[serde(rename = "standardDeviation")]
    pub standard_deviation: f32,
    #[serde(rename = "minutesWatched")]
    pub minutes_watched: i32,
    #[serde(rename = "episodesWatched")]
    pub episodes_watched: i32,
    #[serde(rename = "chaptersRead")]
    pub chapters_read: i32,
    #[serde(rename = "volumesRead")]
    pub volumes_read: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formats: Option<Vec<UserFormatStatistic>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<UserStatusStatistic>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scores: Option<Vec<UserScoreStatistic>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lengths: Option<Vec<UserLengthStatistic>>,
    #[serde(rename = "releaseYears", skip_serializing_if = "Option::is_none")]
    pub release_years: Option<Vec<UserReleaseYearStatistic>>,
    #[serde(rename = "startYears", skip_serializing_if = "Option::is_none")]
    pub start_years: Option<Vec<UserStartYearStatistic>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<UserGenreStatistic>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<UserTagStatistic>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub countries: Option<Vec<UserCountryStatistic>>,
    #[serde(rename = "voiceActors", skip_serializing_if = "Option::is_none")]
    pub voice_actors: Option<Vec<UserVoiceActorStatistic>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staff: Option<Vec<UserStaffStatistic>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studios: Option<Vec<UserStudioStatistic>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserStats {
    #[serde(rename = "watchedTime")]
    pub watched_time: i32,
    #[serde(rename = "chaptersRead")]
    pub chapters_read: i32,
    #[serde(rename = "activityHistory", skip_serializing_if = "Option::is_none")]
    pub activity_history: Option<Vec<UserActivityHistory>>,
    #[serde(rename = "animeStatusDistribution", skip_serializing_if = "Option::is_none")]
    pub anime_status_distribution: Option<Vec<StatusDistribution>>,
    #[serde(rename = "mangaStatusDistribution", skip_serializing_if = "Option::is_none")]
    pub manga_status_distribution: Option<Vec<StatusDistribution>>,
    #[serde(rename = "animeScoreDistribution", skip_serializing_if = "Option::is_none")]
    pub anime_score_distribution: Option<Vec<ScoreDistribution>>,
    #[serde(rename = "mangaScoreDistribution", skip_serializing_if = "Option::is_none")]
    pub manga_score_distribution: Option<Vec<ScoreDistribution>>,
    #[serde(rename = "animeListScores", skip_serializing_if = "Option::is_none")]
    pub anime_list_scores: Option<ListScoreStats>,
    #[serde(rename = "mangaListScores", skip_serializing_if = "Option::is_none")]
    pub manga_list_scores: Option<ListScoreStats>,
    #[serde(rename = "favouredGenresOverview", skip_serializing_if = "Option::is_none")]
    pub favoured_genres_overview: Option<Vec<GenreStats>>,
    #[serde(rename = "favouredGenres", skip_serializing_if = "Option::is_none")]
    pub favoured_genres: Option<Vec<GenreStats>>,
    #[serde(rename = "favouredTags", skip_serializing_if = "Option::is_none")]
    pub favoured_tags: Option<Vec<TagStats>>,
    #[serde(rename = "favouredActors", skip_serializing_if = "Option::is_none")]
    pub favoured_actors: Option<Vec<StaffStats>>,
    #[serde(rename = "favouredStaff", skip_serializing_if = "Option::is_none")]
    pub favoured_staff: Option<Vec<StaffStats>>,
    #[serde(rename = "favouredStudios", skip_serializing_if = "Option::is_none")]
    pub favoured_studios: Option<Vec<StudioStats>>,
    #[serde(rename = "favouredYears", skip_serializing_if = "Option::is_none")]
    pub favoured_years: Option<Vec<YearStats>>,
    #[serde(rename = "favouredFormats", skip_serializing_if = "Option::is_none")]
    pub favoured_formats: Option<Vec<FormatStats>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserStatusStatistic {
    pub count: i32,
    #[serde(rename = "meanScore")]
    pub mean_score: f32,
    #[serde(rename = "minutesWatched")]
    pub minutes_watched: i32,
    #[serde(rename = "chaptersRead")]
    pub chapters_read: i32,
    #[serde(rename = "mediaIds")]
    pub media_ids: Vec<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MediaListStatus>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserStudioStatistic {
    pub count: i32,
    #[serde(rename = "meanScore")]
    pub mean_score: f32,
    #[serde(rename = "minutesWatched")]
    pub minutes_watched: i32,
    #[serde(rename = "chaptersRead")]
    pub chapters_read: i32,
    #[serde(rename = "mediaIds")]
    pub media_ids: Vec<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studio: Option<Studio>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserTagStatistic {
    pub count: i32,
    #[serde(rename = "meanScore")]
    pub mean_score: f32,
    #[serde(rename = "minutesWatched")]
    pub minutes_watched: i32,
    #[serde(rename = "chaptersRead")]
    pub chapters_read: i32,
    #[serde(rename = "mediaIds")]
    pub media_ids: Vec<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<MediaTag>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserVoiceActorStatistic {
    pub count: i32,
    #[serde(rename = "meanScore")]
    pub mean_score: f32,
    #[serde(rename = "minutesWatched")]
    pub minutes_watched: i32,
    #[serde(rename = "chaptersRead")]
    pub chapters_read: i32,
    #[serde(rename = "mediaIds")]
    pub media_ids: Vec<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_actor: Option<Staff>,
    #[serde(rename = "characterIds")]
    pub character_ids: Vec<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct YearStats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    #[serde(rename = "meanScore", skip_serializing_if = "Option::is_none")]
    pub mean_score: Option<i32>,
}
