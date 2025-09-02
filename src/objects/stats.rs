use serde::{Deserialize, Serialize};

use crate::{enums::{media::{CountryCode, MediaFormat}, media_list::MediaListStatus}, objects::{common::PageInfo, media::MediaTag, staff::Staff, studio::Studio, user::UserActivityHistory}};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FormatStats {
    pub format: Option<MediaFormat>,
    pub amount: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenreStats {
    pub genre: Option<String>,
    pub amount: Option<i32>,
    #[serde(rename = "meanScore")]
    pub mean_score: Option<i32>,
    #[serde(rename = "timeWatched")]
    pub time_watched: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListScoreStats {
    #[serde(rename = "meanScore")]
    pub mean_score: Option<i32>,
    #[serde(rename = "standardDeviation")]
    pub standard_deviation: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaStats {
    #[serde(rename = "scoreDistribution")]
    pub score_distribution: Option<Vec<ScoreDistribution>>,
    #[serde(rename = "statusDistribution")]
    pub status_distribution: Option<Vec<StatusDistribution>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScoreDistribution {
    pub score: Option<i32>,
    pub amount: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusDistribution {
    pub status: Option<MediaListStatus>,
    pub amount: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SiteStatistics {
    pub users: Option<SiteTrendConnection>,
    pub anime: Option<SiteTrendConnection>,
    pub manga: Option<SiteTrendConnection>,
    pub characters: Option<SiteTrendConnection>,
    pub staff: Option<SiteTrendConnection>,
    pub studios: Option<SiteTrendConnection>,
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
    pub edges: Option<Vec<SiteTrendEdge>>,
    pub nodes: Option<Vec<SiteTrend>>,
    #[serde(rename = "pageInfo")]
    pub page_info: Option<PageInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SiteTrendEdge {
    pub node: Option<SiteTrend>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StaffStats {
    pub staff: Option<Staff>,
    pub amount: Option<i32>,
    #[serde(rename = "meanScore")]
    pub mean_score: Option<i32>,
    #[serde(rename = "timeWatched")]
    pub time_watched: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StudioStats {
    pub studio: Option<Studio>,
    pub amount: Option<i32>,
    #[serde(rename = "meanScore")]
    pub mean_score: Option<i32>,
    #[serde(rename = "timeWatched")]
    pub time_watched: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagStats {
    pub tag: Option<MediaTag>,
    pub amount: Option<i32>,
    #[serde(rename = "meanScore")]
    pub mean_score: Option<i32>,
    #[serde(rename = "timeWatched")]
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
    #[serde(rename = "releaseYear")]
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
    #[serde(rename = "startYear")]
    pub start_year: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserStatisticTypes {
    #[serde(rename = "animeStatus")]
    pub anime_status: Option<UserStatistics>,
    #[serde(rename = "mangaStatus")]
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
    pub formats: Option<Vec<UserFormatStatistic>>,
    pub statuses: Option<Vec<UserStatusStatistic>>,
    pub scores: Option<Vec<UserScoreStatistic>>,
    pub lengths: Option<Vec<UserLengthStatistic>>,
    #[serde(rename = "releaseYears")]
    pub release_years: Option<Vec<UserReleaseYearStatistic>>,
    #[serde(rename = "startYears")]
    pub start_years: Option<Vec<UserStartYearStatistic>>,
    pub genres: Option<Vec<UserGenreStatistic>>,
    pub tags: Option<Vec<UserTagStatistic>>,
    pub countries: Option<Vec<UserCountryStatistic>>,
    #[serde(rename = "voiceActors")]
    pub voice_actors: Option<Vec<UserVoiceActorStatistic>>,
    pub staff: Option<Vec<UserStaffStatistic>>,
    pub studios: Option<Vec<UserStudioStatistic>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserStats {
    #[serde(rename = "watchedTime")]
    pub watched_time: i32,
    #[serde(rename = "chaptersRead")]
    pub chapters_read: i32,
    #[serde(rename = "activityHistory")]
    pub activity_history: Option<Vec<UserActivityHistory>>,
    #[serde(rename = "animeStatusDistribution")]
    pub anime_status_distribution: Option<Vec<StatusDistribution>>,
    #[serde(rename = "mangaStatusDistribution")]
    pub manga_status_distribution: Option<Vec<StatusDistribution>>,
    #[serde(rename = "animeScoreDistribution")]
    pub anime_score_distribution: Option<Vec<ScoreDistribution>>,
    #[serde(rename = "mangaScoreDistribution")]
    pub manga_score_distribution: Option<Vec<ScoreDistribution>>,
    #[serde(rename = "animeListScores")]
    pub anime_list_scores: Option<ListScoreStats>,
    #[serde(rename = "mangaListScores")]
    pub manga_list_scores: Option<ListScoreStats>,
    #[serde(rename = "favouredGenresOverview")]
    pub favoured_genres_overview: Option<Vec<GenreStats>>,
    #[serde(rename = "favouredGenres")]
    pub favoured_genres: Option<Vec<GenreStats>>,
    #[serde(rename = "favouredTags")]
    pub favoured_tags: Option<Vec<TagStats>>,
    #[serde(rename = "favouredActors")]
    pub favoured_actors: Option<Vec<StaffStats>>,
    #[serde(rename = "favouredStaff")]
    pub favoured_staff: Option<Vec<StaffStats>>,
    #[serde(rename = "favouredStudios")]
    pub favoured_studios: Option<Vec<StudioStats>>,
    #[serde(rename = "favouredYears")]
    pub favoured_years: Option<Vec<YearStats>>,
    #[serde(rename = "favouredFormats")]
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
    pub voice_actor: Option<Staff>,
    #[serde(rename = "characterIds")]
    pub character_ids: Vec<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct YearStats {
    pub year: Option<i32>,
    pub amount: Option<i32>,
    #[serde(rename = "meanScore")]
    pub mean_score: Option<i32>,
}