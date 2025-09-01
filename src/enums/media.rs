use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaFormat {
    #[serde(rename = "TV")]
    Tv,
    #[serde(rename = "TV_SHORT")]
    TvShort,
    #[serde(rename = "MOVIE")]
    Movie,
    #[serde(rename = "SPECIAL")]
    Special,
    #[serde(rename = "OVA")]
    Ova,
    #[serde(rename = "ONA")]
    Ona,
    #[serde(rename = "MUSIC")]
    Music,
    #[serde(rename = "MANGA")]
    Manga,
    #[serde(rename = "NOVEL")]
    Novel,
    #[serde(rename = "ONE_SHOT")]
    OneShot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaRankType {
    #[serde(rename = "RATED")]
    Rated,
    #[serde(rename = "POPULAR")]
    Popular,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaRelation {
    #[serde(rename = "ADAPTATION")]
    Adaptation,
    #[serde(rename = "PREQUEL")]
    Prequel,
    #[serde(rename = "SEQUEL")]
    Sequel,
    #[serde(rename = "PARENT")]
    Parent,
    #[serde(rename = "SIDE_STORY")]
    SideStory,
    #[serde(rename = "CHARACTER")]
    Character,
    #[serde(rename = "SUMMARY")]
    Summary,
    #[serde(rename = "ALTERNATIVE")]
    Alternative,
    #[serde(rename = "SPIN_OFF")]
    SpinOff,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "SOURCE")]
    Source,
    #[serde(rename = "COMPILATION")]
    Compilation,
    #[serde(rename = "CONTAINS")]
    Contains,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaSeason {
    #[serde(rename = "WINTER")]
    Winter,
    #[serde(rename = "SPRING")]
    Spring,
    #[serde(rename = "SUMMER")]
    Summer,
    #[serde(rename = "FALL")]
    Fall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaSort {
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "ID_DESC")]
    IdDesc,
    #[serde(rename = "TITLE_ROMAJI")]
    TitleRomaji,
    #[serde(rename = "TITLE_ROMAJI_DESC")]
    TitleRomajiDesc,
    #[serde(rename = "TITLE_ENGLISH")]
    TitleEnglish,
    #[serde(rename = "TITLE_ENGLISH_DESC")]
    TitleEnglishDesc,
    #[serde(rename = "TITLE_NATIVE")]
    TitleNative,
    #[serde(rename = "TITLE_NATIVE_DESC")]
    TitleNativeDesc,
    #[serde(rename = "TYPE")]
    Type,
    #[serde(rename = "TYPE_DESC")]
    TypeDesc,
    #[serde(rename = "FORMAT")]
    Format,
    #[serde(rename = "FORMAT_DESC")]
    FormatDesc,
    #[serde(rename = "START_DATE")]
    StartDate,
    #[serde(rename = "START_DATE_DESC")]
    StartDateDesc,
    #[serde(rename = "END_DATE")]
    EndDate,
    #[serde(rename = "END_DATE_DESC")]
    EndDateDesc,
    #[serde(rename = "SCORE")]
    Score,
    #[serde(rename = "SCORE_DESC")]
    ScoreDesc,
    #[serde(rename = "POPULARITY")]
    Popularity,
    #[serde(rename = "POPULARITY_DESC")]
    PopularityDesc,
    #[serde(rename = "TRENDING")]
    Trending,
    #[serde(rename = "TRENDING_DESC")]
    TrendingDesc,
    #[serde(rename = "EPISODES")]
    Episodes,
    #[serde(rename = "EPISODES_DESC")]
    EpisodesDesc,
    #[serde(rename = "DURATION")]
    Duration,
    #[serde(rename = "DURATION_DESC")]
    DurationDesc,
    #[serde(rename = "STATUS")]
    Status,
    #[serde(rename = "STATUS_DESC")]
    StatusDesc,
    #[serde(rename = "CHAPTERS")]
    Chapters,
    #[serde(rename = "CHAPTERS_DESC")]
    ChaptersDesc,
    #[serde(rename = "VOLUMES")]
    Volumes,
    #[serde(rename = "VOLUMES_DESC")]
    VolumesDesc,
    #[serde(rename = "UPDATED_AT")]
    UpdatedAt,
    #[serde(rename = "UPDATED_AT_DESC")]
    UpdatedAtDesc,
    #[serde(rename = "SEARCH_MATCH")]
    SearchMatch,
    #[serde(rename = "FAVOURITES")]
    Favourites,
    #[serde(rename = "FAVOURITES_DESC")]
    FavouritesDesc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaSource {
    #[serde(rename = "ORIGINAL")]
    Original,
    #[serde(rename = "MANGA")]
    Manga,
    #[serde(rename = "LIGHT_NOVEL")]
    LightNovel,
    #[serde(rename = "VISUAL_NOVEL")]
    VisualNovel,
    #[serde(rename = "VIDEO_GAME")]
    VideoGame,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "NOVEL")]
    Novel,
    #[serde(rename = "DOUJINSHI")]
    Doujinshi,
    #[serde(rename = "ANIME")]
    Anime,
    #[serde(rename = "WEB_NOVEL")]
    WebNovel,
    #[serde(rename = "LIVE_ACTION")]
    LiveAction,
    #[serde(rename = "GAME")]
    Game,
    #[serde(rename = "COMIC")]
    Comic,
    #[serde(rename = "MULTIMEDIA_PROJECT")]
    MultimediaProject,
    #[serde(rename = "PICTURE_BOOK")]
    PictureBook,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaStatus {
    #[serde(rename = "FINISHED")]
    Finished,
    #[serde(rename = "RELEASING")]
    Releasing,
    #[serde(rename = "NOT_YET_RELEASED")]
    NotYetReleased,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "HIATUS")]
    Hiatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaTrendSort {
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "ID_DESC")]
    IdDesc,
    #[serde(rename = "MEDIA_ID")]
    MediaId,
    #[serde(rename = "MEDIA_ID_DESC")]
    MediaIdDesc,
    #[serde(rename = "DATE")]
    Date,
    #[serde(rename = "DATE_DESC")]
    DateDesc,
    #[serde(rename = "SCORE")]
    Score,
    #[serde(rename = "SCORE_DESC")]
    ScoreDesc,
    #[serde(rename = "POPULARITY")]
    Popularity,
    #[serde(rename = "POPULARITY_DESC")]
    PopularityDesc,
    #[serde(rename = "TRENDING")]
    Trending,
    #[serde(rename = "TRENDING_DESC")]
    TrendingDesc,
    #[serde(rename = "EPISODE")]
    Episode,
    #[serde(rename = "EPISODE_DESC")]
    EpisodeDesc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaType {
    #[serde(rename = "ANIME")]
    Anime,
    #[serde(rename = "MANGA")]
    Manga,
}