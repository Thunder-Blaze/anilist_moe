use crate::enums::media::{
    CountryCode, MediaFormat, MediaSeason, MediaSort, MediaStatus, MediaType,
};
use crate::enums::staff::StaffLanguage;
use crate::errors::AniListError;
use crate::objects::media::Media;
use crate::objects::responses::Page;
use crate::options;
#[cfg(feature = "pagination")]
use crate::pagination::Paginated;
use crate::{client::AniListClient, queries::media};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::borrow::Cow;

/// Fetch media (anime/manga) with filters and pagination.
///
/// Example:
/// ```rust
/// # use anilist_moe::endpoints::media::FetchMediaOptions;
/// # use anilist_moe::enums::media::{MediaType, MediaSeason, MediaSort};
/// # use anilist_moe::options;
/// let options = options!(
///     FetchMediaOptions,
///     media_type: MediaType::Anime,
///     season: MediaSeason::Fall,
///     season_year: 2024,
///     sort: &[MediaSort::Popularity],
///     page: 1,
///     per_page: 20,
/// );
/// ```
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchMediaOptions<'a> {
    pub id: Option<u32>,
    pub id_mal: Option<u32>,
    pub search: Option<Cow<'a, str>>,
    pub media_type: Option<MediaType>,
    pub format: Option<MediaFormat>,
    pub status: Option<MediaStatus>,
    pub season: Option<MediaSeason>,
    pub season_year: Option<i32>,
    pub genre: Option<Cow<'a, str>>,
    pub tag: Option<Cow<'a, str>>,
    pub tag_category: Option<Cow<'a, str>>,
    pub minimum_tag_rank: Option<i32>,
    pub source: Option<Cow<'a, str>>,
    pub country_of_origin: Option<Cow<'a, str>>,
    pub country_of_origin_in: Option<Cow<'a, [CountryCode]>>,
    pub country_of_origin_not_in: Option<Cow<'a, [CountryCode]>>,
    pub is_licensed: Option<bool>,
    pub is_adult: Option<bool>,
    pub on_list: Option<bool>,
    pub licensed_by: Option<Cow<'a, str>>,
    pub licensed_by_id: Option<u32>,
    pub id_not: Option<u32>,
    pub id_in: Option<Cow<'a, [u32]>>,
    pub id_not_in: Option<Cow<'a, [u32]>>,
    pub id_mal_not: Option<u32>,
    pub id_mal_in: Option<Cow<'a, [u32]>>,
    pub id_mal_not_in: Option<Cow<'a, [u32]>>,
    pub format_not: Option<MediaFormat>,
    pub format_in: Option<Cow<'a, [MediaFormat]>>,
    pub format_not_in: Option<Cow<'a, [MediaFormat]>>,
    pub status_not: Option<MediaStatus>,
    pub status_in: Option<Cow<'a, [MediaStatus]>>,
    pub status_not_in: Option<Cow<'a, [MediaStatus]>>,
    pub genre_in: Option<Cow<'a, [&'a str]>>,
    pub genre_not_in: Option<Cow<'a, [&'a str]>>,
    pub tag_in: Option<Cow<'a, [&'a str]>>,
    pub tag_not_in: Option<Cow<'a, [&'a str]>>,
    pub tag_category_in: Option<Cow<'a, [&'a str]>>,
    pub tag_category_not_in: Option<Cow<'a, [&'a str]>>,
    pub licensed_by_in: Option<Cow<'a, [&'a str]>>,
    pub licensed_by_id_in: Option<Cow<'a, [u32]>>,
    pub source_in: Option<Cow<'a, [&'a str]>>,
    pub start_date: Option<i32>,
    pub end_date: Option<i32>,
    pub start_date_greater: Option<i32>,
    pub start_date_lesser: Option<i32>,
    pub start_date_like: Option<Cow<'a, str>>,
    pub end_date_greater: Option<i32>,
    pub end_date_lesser: Option<i32>,
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
    pub sort: Option<Cow<'a, [MediaSort]>>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
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
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchMediaOneOptions<'a> {
    pub id: Option<u32>,
    pub id_mal: Option<u32>,
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
    /// # use anilist_moe::options;
    /// # async fn example() -> Result<(), AniListError> {
    /// let client = AniListClient::new();
    /// let options = options!(
    ///     FetchMediaOptions,
    ///     media_type: MediaType::Anime,
    ///     sort: &[MediaSort::Popularity],
    ///     page: 1,
    ///     per_page: 10,
    /// );
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
    /// let options = FetchMediaOneOptions { id: Some(16498), ..Default::default() };
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

    // Convenience functions - Anime

    /// Get popular anime
    pub async fn get_popular_anime(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(&options!(
            FetchMediaOptions,
            media_type: Some(MediaType::Anime),
            sort: Some(&[MediaSort::PopularityDesc]),
            include_start_date: Some(true),
            include_end_date: Some(true),
            page,
            per_page,
        ))
        .await
    }

    /// Get trending anime
    pub async fn get_trending_anime(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(&options!(
            FetchMediaOptions,
            media_type: Some(MediaType::Anime),
            sort: Some(&[MediaSort::TrendingDesc]),
            include_start_date: Some(true),
            include_end_date: Some(true),
            page,
            per_page,
        ))
        .await
    }

    /// Get currently airing anime
    pub async fn get_airing_anime(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(&options!(
            FetchMediaOptions,
            media_type: Some(MediaType::Anime),
            status: Some(MediaStatus::Releasing),
            sort: Some(&[MediaSort::PopularityDesc]),
            include_start_date: Some(true),
            include_end_date: Some(true),
            page,
            per_page,
        ))
        .await
    }

    /// Get upcoming anime (not yet released)
    pub async fn get_upcoming_anime(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(&options!(
            FetchMediaOptions,
            media_type: Some(MediaType::Anime),
            status: Some(MediaStatus::NotYetReleased),
            sort: Some(&[MediaSort::PopularityDesc]),
            page,
            per_page,
        ))
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
        self.fetch(&options!(
            FetchMediaOptions,
            media_type: Some(MediaType::Anime),
            season: Some(season),
            season_year: Some(year),
            sort: Some(&[MediaSort::PopularityDesc]),
            include_start_date: Some(true),
            include_end_date: Some(true),
            page,
            per_page,
        ))
        .await
    }

    /// Search anime by title
    pub async fn search_anime(
        &self,
        query: &str,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(&options!(
            FetchMediaOptions,
            media_type: Some(MediaType::Anime),
            search: Some(query),
            sort: Some(&[MediaSort::SearchMatch]),
            include_start_date: Some(true),
            include_end_date: Some(true),
            page,
            per_page,
        ))
        .await
    }

    /// Get anime by ID
    pub async fn get_anime_by_id(&self, id: u32) -> Result<Media, AniListError> {
        self.fetch_one(&options!(
            FetchMediaOneOptions,
            id: Some(id),
        ))
        .await
    }

    /// Get top rated anime
    pub async fn get_top_rated_anime(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(&options!(
            FetchMediaOptions,
            media_type: Some(MediaType::Anime),
            sort: Some(&[MediaSort::ScoreDesc]),
            include_start_date: Some(true),
            include_end_date: Some(true),
            page,
            per_page,
        ))
        .await
    }

    // Convenience functions - Manga

    /// Get popular manga
    pub async fn get_popular_manga(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(&options!(
            FetchMediaOptions,
            media_type: Some(MediaType::Manga),
            sort: Some(&[MediaSort::PopularityDesc]),
            include_start_date: Some(true),
            include_end_date: Some(true),
            page,
            per_page,
        ))
        .await
    }

    /// Get trending manga
    pub async fn get_trending_manga(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(&options!(
            FetchMediaOptions,
            media_type: Some(MediaType::Manga),
            sort: Some(&[MediaSort::TrendingDesc]),
            include_start_date: Some(true),
            include_end_date: Some(true),
            page,
            per_page,
        ))
        .await
    }

    /// Get currently releasing manga
    pub async fn get_releasing_manga(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(&options!(
            FetchMediaOptions,
            media_type: Some(MediaType::Manga),
            status: Some(MediaStatus::Releasing),
            sort: Some(&[MediaSort::PopularityDesc]),
            include_start_date: Some(true),
            include_end_date: Some(true),
            page,
            per_page,
        ))
        .await
    }

    /// Get completed manga
    pub async fn get_completed_manga(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(&options!(
            FetchMediaOptions,
            media_type: Some(MediaType::Manga),
            status: Some(MediaStatus::Finished),
            sort: Some(&[MediaSort::PopularityDesc]),
            include_start_date: Some(true),
            include_end_date: Some(true),
            page,
            per_page,
        ))
        .await
    }

    /// Search manga by title
    pub async fn search_manga(
        &self,
        query: &str,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(&options!(
            FetchMediaOptions,
            media_type: Some(MediaType::Manga),
            search: Some(query),
            sort: Some(&[MediaSort::SearchMatch]),
            include_start_date: Some(true),
            include_end_date: Some(true),
            page,
            per_page,
        ))
        .await
    }

    /// Get manga by ID
    pub async fn get_manga_by_id(&self, id: u32) -> Result<Media, AniListError> {
        self.fetch_one(&options!(
            FetchMediaOneOptions,
            id: Some(id),
        ))
        .await
    }

    /// Get top rated manga
    pub async fn get_top_rated_manga(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(&options!(
            FetchMediaOptions,
            media_type: Some(MediaType::Manga),
            sort: Some(&[MediaSort::ScoreDesc]),
            page,
            per_page,
        ))
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
