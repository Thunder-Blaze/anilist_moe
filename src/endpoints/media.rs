use crate::enums::media::{
    CountryCode, MediaFormat, MediaSeason, MediaSort, MediaStatus, MediaTrendSort, MediaType,
};
use crate::enums::staff::StaffLanguage;
use crate::errors::AniListError;
use crate::objects::media::{Media, MediaTrend};
use crate::objects::responses::Page;
#[cfg(feature = "pagination")]
use crate::pagination::Paginated;
use crate::{client::AniListClient, queries::media};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::borrow::Cow;
use typed_builder::TypedBuilder;

/// Fetch media (anime/manga) with filters and pagination.
///
/// Example:
/// ```rust
/// # use anilist_moe::endpoints::media::FetchMediaOptions;
/// # use anilist_moe::enums::media::{MediaType, MediaSeason, MediaSort};
/// # use std::borrow::Cow;
/// let options = FetchMediaOptions::builder()
///     .media_type(MediaType::Anime)
///     .season(MediaSeason::Fall)
///     .season_year(2024)
///     .sort(&[MediaSort::Popularity])
///     .page_opt(Some(1))
///     .per_page_opt(Some(20))
///     .build();
/// ```
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(
    default,
    setter(strip_option(ignore_invalid, fallback_suffix = "_opt"))
))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FetchMediaOptions<'a> {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub id: Option<u32>,
    pub id_mal: Option<u32>,
    #[builder(setter(into))]
    pub search: Option<Cow<'a, str>>,
    pub media_type: Option<MediaType>,
    pub format: Option<MediaFormat>,
    pub status: Option<MediaStatus>,
    pub season: Option<MediaSeason>,
    pub season_year: Option<i32>,
    #[builder(setter(into))]
    pub genre: Option<Cow<'a, str>>,
    #[builder(setter(into))]
    pub tag: Option<Cow<'a, str>>,
    #[builder(setter(into))]
    pub tag_category: Option<Cow<'a, str>>,
    pub minimum_tag_rank: Option<i32>,
    #[builder(setter(into))]
    pub source: Option<Cow<'a, str>>,
    #[builder(setter(into))]
    pub country_of_origin: Option<Cow<'a, str>>,
    #[builder(setter(into))]
    pub country_of_origin_in: Option<Cow<'a, [CountryCode]>>,
    #[builder(setter(into))]
    pub country_of_origin_not_in: Option<Cow<'a, [CountryCode]>>,
    pub is_licensed: Option<bool>,
    pub is_adult: Option<bool>,
    pub on_list: Option<bool>,
    #[builder(setter(into))]
    pub licensed_by: Option<Cow<'a, str>>,
    pub licensed_by_id: Option<u32>,
    pub id_not: Option<u32>,
    #[builder(setter(into))]
    pub id_in: Option<Cow<'a, [u32]>>,
    #[builder(setter(into))]
    pub id_not_in: Option<Cow<'a, [u32]>>,
    pub id_mal_not: Option<u32>,
    #[builder(setter(into))]
    pub id_mal_in: Option<Cow<'a, [u32]>>,
    #[builder(setter(into))]
    pub id_mal_not_in: Option<Cow<'a, [u32]>>,
    pub format_not: Option<MediaFormat>,
    #[builder(setter(into))]
    pub format_in: Option<Cow<'a, [MediaFormat]>>,
    #[builder(setter(into))]
    pub format_not_in: Option<Cow<'a, [MediaFormat]>>,
    pub status_not: Option<MediaStatus>,
    #[builder(setter(into))]
    pub status_in: Option<Cow<'a, [MediaStatus]>>,
    #[builder(setter(into))]
    pub status_not_in: Option<Cow<'a, [MediaStatus]>>,
    #[builder(setter(into))]
    pub genre_in: Option<Cow<'a, [&'a str]>>,
    #[builder(setter(into))]
    pub genre_not_in: Option<Cow<'a, [&'a str]>>,
    #[builder(setter(into))]
    pub tag_in: Option<Cow<'a, [&'a str]>>,
    #[builder(setter(into))]
    pub tag_not_in: Option<Cow<'a, [&'a str]>>,
    #[builder(setter(into))]
    pub tag_category_in: Option<Cow<'a, [&'a str]>>,
    #[builder(setter(into))]
    pub tag_category_not_in: Option<Cow<'a, [&'a str]>>,
    #[builder(setter(into))]
    pub licensed_by_in: Option<Cow<'a, [&'a str]>>,
    #[builder(setter(into))]
    pub licensed_by_id_in: Option<Cow<'a, [u32]>>,
    #[builder(setter(into))]
    pub source_in: Option<Cow<'a, [&'a str]>>,
    pub start_date: Option<i32>,
    pub end_date: Option<i32>,
    pub start_date_greater: Option<i32>,
    pub start_date_lesser: Option<i32>,
    #[builder(setter(into))]
    pub start_date_like: Option<Cow<'a, str>>,
    pub end_date_greater: Option<i32>,
    pub end_date_lesser: Option<i32>,
    #[builder(setter(into))]
    pub end_date_like: Option<Cow<'a, str>>,
    pub average_score: Option<i32>,
    pub average_score_not: Option<i32>,
    pub average_score_greater: Option<i32>,
    pub average_score_lesser: Option<i32>,
    pub popularity: Option<i32>,
    pub popularity_not: Option<i32>,
    pub popularity_greater: Option<i32>,
    pub popularity_lesser: Option<i32>,
    pub episodes: Option<i32>,
    pub episodes_greater: Option<i32>,
    pub episodes_lesser: Option<i32>,
    pub duration: Option<i32>,
    pub duration_greater: Option<i32>,
    pub duration_lesser: Option<i32>,
    pub chapters: Option<i32>,
    pub chapters_greater: Option<i32>,
    pub chapters_lesser: Option<i32>,
    pub volumes: Option<i32>,
    pub volumes_greater: Option<i32>,
    pub volumes_lesser: Option<i32>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset sorts by the same key alone.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "crate::options::skip_never"
    )]
    #[builder(setter(into))]
    pub sort: Option<Cow<'a, [MediaSort]>>,
    pub include_start_date: Option<bool>,
    pub include_end_date: Option<bool>,
    pub include_duration: Option<bool>,
    pub include_volumes: Option<bool>,
    pub include_country_of_origin: Option<bool>,
    pub include_is_licensed: Option<bool>,
    pub include_source: Option<bool>,
    pub include_hashtag: Option<bool>,
    pub include_trailer: Option<bool>,
    pub include_updated_at: Option<bool>,
    pub include_cover_image_extra_large: Option<bool>,
    pub include_mean_score: Option<bool>,
    pub include_is_locked: Option<bool>,
    pub include_trending: Option<bool>,
    pub include_tags: Option<bool>,
    pub include_next_airing_episode: Option<bool>,
    pub include_relations: Option<bool>,
    pub include_characters: Option<bool>,
    pub include_staff: Option<bool>,
    pub include_studios: Option<bool>,
    pub include_media_list_entry_details: Option<bool>,
    pub include_reviews: Option<bool>,
    pub include_recommendations: Option<bool>,
    pub include_external_links: Option<bool>,
    pub include_streaming_episodes: Option<bool>,
    pub include_rankings: Option<bool>,
    pub include_stats: Option<bool>,
    pub include_auto_create_forum_thread: Option<bool>,
    pub include_is_recommendation_blocked: Option<bool>,
    pub include_is_review_blocked: Option<bool>,
    pub include_mod_notes: Option<bool>,
    // Pagination fields
    pub characters_page: Option<i32>,
    pub characters_per_page: Option<i32>,
    pub staff_page: Option<i32>,
    pub staff_per_page: Option<i32>,
    pub reviews_page: Option<i32>,
    pub reviews_per_page: Option<i32>,
    pub recommendations_page: Option<i32>,
    pub recommendations_per_page: Option<i32>,
    // Filter fields for included connections
    pub voice_actor_language: Option<StaffLanguage>,
    // HTML rendering options
    pub description_as_html: Option<bool>,
}

/// Fetch detailed info for a single media item.
///
/// Supports optional pagination for related data (characters, staff, reviews, recommendations).
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(
    default,
    setter(strip_option(ignore_invalid, fallback_suffix = "_opt"))
))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FetchMediaOneOptions<'a> {
    pub id: Option<u32>,
    pub id_mal: Option<u32>,
    #[builder(setter(into))]
    pub search: Option<Cow<'a, str>>,
    // Characters pagination
    pub fetch_characters: Option<bool>,
    pub characters_page: Option<i32>,
    pub characters_per_page: Option<i32>,
    // Staff pagination
    pub fetch_staff: Option<bool>,
    pub staff_page: Option<i32>,
    pub staff_per_page: Option<i32>,
    // Reviews pagination
    pub fetch_reviews: Option<bool>,
    pub reviews_page: Option<i32>,
    pub reviews_per_page: Option<i32>,
    // Recommendations pagination
    pub fetch_recommendations: Option<bool>,
    pub recommendations_page: Option<i32>,
    pub recommendations_per_page: Option<i32>,
}

/// Anime and manga endpoint.
///
/// Examples:
/// ```rust
/// # use anilist_moe::AniListClient;
/// # use anilist_moe::AniListError;
/// # async fn example() -> Result<(), AniListError> {
/// let client = AniListClient::new();
/// let trending = client.media().get_trending_anime(Some(1), Some(10)).await?;
/// let results = client.media().search_anime("Steins Gate", Some(1), Some(5)).await?;
/// let anime = client.media().get_anime_by_id(16498).await?;
/// # Ok(())
/// # }
/// ```
/// Options for the media trend history of a title.
///
/// This is the daily record AniList keeps for a media: its trending score,
/// average score, popularity and episode count, one row per day.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(
    default,
    setter(strip_option(ignore_invalid, fallback_suffix = "_opt"))
))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FetchMediaTrendOptions<'a> {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub media_id: Option<u32>,
    pub date: Option<i32>,
    pub trending: Option<i32>,
    pub average_score: Option<i32>,
    pub popularity: Option<i32>,
    pub episode: Option<i32>,
    pub releasing: Option<bool>,
    pub media_id_not: Option<u32>,
    #[builder(setter(into))]
    pub media_id_in: Option<Cow<'a, [u32]>>,
    #[builder(setter(into))]
    pub media_id_not_in: Option<Cow<'a, [u32]>>,
    pub date_greater: Option<i32>,
    pub date_lesser: Option<i32>,
    pub trending_greater: Option<i32>,
    pub trending_lesser: Option<i32>,
    pub trending_not: Option<i32>,
    pub average_score_greater: Option<i32>,
    pub average_score_lesser: Option<i32>,
    pub average_score_not: Option<i32>,
    pub popularity_greater: Option<i32>,
    pub popularity_lesser: Option<i32>,
    pub popularity_not: Option<i32>,
    pub episode_greater: Option<i32>,
    pub episode_lesser: Option<i32>,
    pub episode_not: Option<i32>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset keeps the order the query document asks for.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(setter(into))]
    pub sort: Option<Cow<'a, [MediaTrendSort]>>,
}

pub struct MediaEndpoint<'a> {
    client: &'a AniListClient,
}

impl<'a> MediaEndpoint<'a> {
    /// Creates a new MediaEndpoint instance.
    pub fn new(client: &'a AniListClient) -> Self {
        Self { client }
    }

    /// Fetch a page of media (anime/manga).
    ///
    /// Example:
    /// ```rust
    /// # use anilist_moe::AniListClient;
    /// # use anilist_moe::AniListError;
    /// # use anilist_moe::endpoints::media::FetchMediaOptions;
    /// # use anilist_moe::enums::media::{MediaType, MediaSort};
    /// # use std::borrow::Cow;
    ///
    /// # async fn example() -> Result<(), AniListError> {
    /// let client = AniListClient::new();
    /// let options = FetchMediaOptions::builder()
    ///     .media_type(MediaType::Anime)
    ///     .sort(&[MediaSort::Popularity])
    ///     .page_opt(Some(1))
    ///     .per_page_opt(Some(10))
    ///     .build();
    ///
    /// let response = client.media().fetch(&options).await?;
    /// for anime in &response.data { println!("Title: {:?}", anime.title); }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch(
        &self,
        options: &FetchMediaOptions<'_>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        let query = media::FETCH;
        self.client.fetch(query, Some(&options)).await
    }

    /// Fetch a single media item.
    ///
    /// Example:
    /// ```rust
    /// # use anilist_moe::AniListClient;
    /// # use anilist_moe::AniListError;
    /// # use anilist_moe::endpoints::media::FetchMediaOneOptions;
    /// # async fn example() -> Result<(), AniListError> {
    /// let client = AniListClient::new();
    /// let options = FetchMediaOneOptions::builder().id(16498).build();
    /// let anime = client.media().fetch_one(&options).await?;
    /// println!("Title: {:?}", anime.title);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_one(
        &self,
        options: &FetchMediaOneOptions<'_>,
    ) -> Result<Media, AniListError> {
        let query = media::FETCH_ONE;
        self.client.fetch(query, Some(&options)).await
    }

    /// Fetch the daily trend history of a media, one page at a time.
    pub async fn fetch_trends(
        &self,
        options: &FetchMediaTrendOptions<'_>,
    ) -> Result<Page<Vec<MediaTrend>>, AniListError> {
        let query = media::FETCH_TRENDS;
        self.client.fetch(query, Some(&options)).await
    }

    /// Fetch one day of a media's trend history.
    ///
    /// The options are the ones [`fetch_trends`](Self::fetch_trends) takes;
    /// `page` and `per_page` are ignored, because the API answers with a single
    /// day here.
    pub async fn fetch_trend_one(
        &self,
        options: &FetchMediaTrendOptions<'_>,
    ) -> Result<MediaTrend, AniListError> {
        let query = media::FETCH_TREND_ONE;
        self.client.fetch(query, Some(&options)).await
    }

    // Convenience functions - Anime

    /// Get popular anime
    pub async fn get_popular_anime(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(
            &FetchMediaOptions::builder()
                .media_type(MediaType::Anime)
                .sort(&[MediaSort::PopularityDesc])
                .include_start_date(true)
                .include_end_date(true)
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
        .await
    }

    /// Get trending anime
    pub async fn get_trending_anime(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(
            &FetchMediaOptions::builder()
                .media_type(MediaType::Anime)
                .sort(&[MediaSort::TrendingDesc])
                .include_start_date(true)
                .include_end_date(true)
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
        .await
    }

    /// Get currently airing anime
    pub async fn get_airing_anime(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(
            &FetchMediaOptions::builder()
                .media_type(MediaType::Anime)
                .status(MediaStatus::Releasing)
                .sort(&[MediaSort::PopularityDesc])
                .include_start_date(true)
                .include_end_date(true)
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
        .await
    }

    /// Get upcoming anime (not yet released)
    pub async fn get_upcoming_anime(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(
            &FetchMediaOptions::builder()
                .media_type(MediaType::Anime)
                .status(MediaStatus::NotYetReleased)
                .sort(&[MediaSort::PopularityDesc])
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
        .await
    }

    /// Get anime by season and year
    pub async fn get_by_season(
        &self,
        season: MediaSeason,
        year: i32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(
            &FetchMediaOptions::builder()
                .media_type(MediaType::Anime)
                .season(season)
                .season_year(year)
                .sort(&[MediaSort::PopularityDesc])
                .include_start_date(true)
                .include_end_date(true)
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
        .await
    }

    /// Search anime by title
    pub async fn search_anime(
        &self,
        query: &str,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(
            &FetchMediaOptions::builder()
                .media_type(MediaType::Anime)
                .search(Cow::Borrowed(query))
                .sort(&[MediaSort::SearchMatch])
                .include_start_date(true)
                .include_end_date(true)
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
        .await
    }

    /// Get anime by ID
    pub async fn get_anime_by_id(&self, id: u32) -> Result<Media, AniListError> {
        self.fetch_one(&FetchMediaOneOptions::builder().id(id).build())
            .await
    }

    /// Get top rated anime
    pub async fn get_top_rated_anime(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(
            &FetchMediaOptions::builder()
                .media_type(MediaType::Anime)
                .sort(&[MediaSort::ScoreDesc])
                .include_start_date(true)
                .include_end_date(true)
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
        .await
    }

    // Convenience functions - Manga

    /// Get popular manga
    pub async fn get_popular_manga(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(
            &FetchMediaOptions::builder()
                .media_type(MediaType::Manga)
                .sort(&[MediaSort::PopularityDesc])
                .include_start_date(true)
                .include_end_date(true)
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
        .await
    }

    /// Get trending manga
    pub async fn get_trending_manga(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(
            &FetchMediaOptions::builder()
                .media_type(MediaType::Manga)
                .sort(&[MediaSort::TrendingDesc])
                .include_start_date(true)
                .include_end_date(true)
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
        .await
    }

    /// Get currently releasing manga
    pub async fn get_releasing_manga(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(
            &FetchMediaOptions::builder()
                .media_type(MediaType::Manga)
                .status(MediaStatus::Releasing)
                .sort(&[MediaSort::PopularityDesc])
                .include_start_date(true)
                .include_end_date(true)
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
        .await
    }

    /// Get completed manga
    pub async fn get_completed_manga(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(
            &FetchMediaOptions::builder()
                .media_type(MediaType::Manga)
                .status(MediaStatus::Finished)
                .sort(&[MediaSort::PopularityDesc])
                .include_start_date(true)
                .include_end_date(true)
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
        .await
    }

    /// Search manga by title
    pub async fn search_manga(
        &self,
        query: &str,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(
            &FetchMediaOptions::builder()
                .media_type(MediaType::Manga)
                .search(Cow::Borrowed(query))
                .sort(&[MediaSort::SearchMatch])
                .include_start_date(true)
                .include_end_date(true)
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
        .await
    }

    /// Get manga by ID
    pub async fn get_manga_by_id(&self, id: u32) -> Result<Media, AniListError> {
        self.fetch_one(&FetchMediaOneOptions::builder().id(id).build())
            .await
    }

    /// Get top rated manga
    pub async fn get_top_rated_manga(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(
            &FetchMediaOptions::builder()
                .media_type(MediaType::Manga)
                .sort(&[MediaSort::ScoreDesc])
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
        .await
    }
}

#[cfg(feature = "pagination")]
impl<'a> Paginated for FetchMediaOptions<'a> {
    type Item = Media;

    async fn fetch(&self, client: &AniListClient) -> Result<Page<Vec<Media>>, AniListError> {
        client.media().fetch(self).await
    }

    fn set_page(&mut self, page: i32) {
        self.page = Some(page);
    }
}
