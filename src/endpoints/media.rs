use crate::enums::media::{MediaFormat, MediaSeason, MediaSort, MediaStatus, MediaType};
use crate::errors::AniListError;
use crate::objects::media::Media;
use crate::objects::responses::{GraphQLResponse, Page};
use crate::{client::AniListClient, queries::media};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;

/// Options for fetching media (anime/manga) with various filters.
///
/// This struct provides comprehensive filtering and pagination options
/// for querying anime and manga from the AniList API.
///
/// # Examples
///
/// ```rust
/// # use anilist_moe::endpoints::media::FetchMediaOptions;
/// # use anilist_moe::enums::media::{MediaType, MediaSeason, MediaSort};
/// let options = FetchMediaOptions {
///     media_type: Some(MediaType::Anime),
///     season: Some(MediaSeason::Fall),
///     season_year: Some(2024),
///     sort: Some(vec![MediaSort::Popularity]),
///     page: Some(1),
///     per_page: Some(20),
///     ..Default::default()
/// };
/// ```
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FetchMediaOptions {
    pub id: Option<i32>,
    #[serde(rename = "idMal")]
    pub id_mal: Option<i32>,
    pub search: Option<String>,
    #[serde(rename = "type")]
    pub media_type: Option<MediaType>,
    pub format: Option<MediaFormat>,
    pub status: Option<MediaStatus>,
    pub season: Option<MediaSeason>,
    #[serde(rename = "seasonYear")]
    pub season_year: Option<i32>,
    pub genre: Option<String>,
    pub tag: Option<String>,
    #[serde(rename = "tagCategory")]
    pub tag_category: Option<String>,
    #[serde(rename = "minimumTagRank")]
    pub minimum_tag_rank: Option<i32>,
    pub source: Option<String>,
    #[serde(rename = "countryOfOrigin")]
    pub country_of_origin: Option<String>,
    #[serde(rename = "isLicensed")]
    pub is_licensed: Option<bool>,
    #[serde(rename = "isAdult")]
    pub is_adult: Option<bool>,
    #[serde(rename = "onList")]
    pub on_list: Option<bool>,
    #[serde(rename = "licensedBy")]
    pub licensed_by: Option<String>,
    #[serde(rename = "licensedById")]
    pub licensed_by_id: Option<i32>,
    pub id_not: Option<i32>,
    pub id_in: Option<Vec<i32>>,
    pub id_not_in: Option<Vec<i32>>,
    #[serde(rename = "idMal_not")]
    pub id_mal_not: Option<i32>,
    #[serde(rename = "idMal_in")]
    pub id_mal_in: Option<Vec<i32>>,
    #[serde(rename = "idMal_not_in")]
    pub id_mal_not_in: Option<Vec<i32>>,
    pub format_not: Option<MediaFormat>,
    pub format_in: Option<Vec<MediaFormat>>,
    pub format_not_in: Option<Vec<MediaFormat>>,
    pub status_not: Option<MediaStatus>,
    pub status_in: Option<Vec<MediaStatus>>,
    pub status_not_in: Option<Vec<MediaStatus>>,
    pub genre_in: Option<Vec<String>>,
    pub genre_not_in: Option<Vec<String>>,
    pub tag_in: Option<Vec<String>>,
    pub tag_not_in: Option<Vec<String>>,
    #[serde(rename = "tagCategory_in")]
    pub tag_category_in: Option<Vec<String>>,
    #[serde(rename = "tagCategory_not_in")]
    pub tag_category_not_in: Option<Vec<String>>,
    #[serde(rename = "licensedBy_in")]
    pub licensed_by_in: Option<Vec<String>>,
    #[serde(rename = "licensedById_in")]
    pub licensed_by_id_in: Option<Vec<i32>>,
    pub source_in: Option<Vec<String>>,
    #[serde(rename = "startDate")]
    pub start_date: Option<i32>,
    #[serde(rename = "endDate")]
    pub end_date: Option<i32>,
    #[serde(rename = "startDate_greater")]
    pub start_date_greater: Option<i32>,
    #[serde(rename = "startDate_lesser")]
    pub start_date_lesser: Option<i32>,
    #[serde(rename = "startDate_like")]
    pub start_date_like: Option<String>,
    #[serde(rename = "endDate_greater")]
    pub end_date_greater: Option<i32>,
    #[serde(rename = "endDate_lesser")]
    pub end_date_lesser: Option<i32>,
    #[serde(rename = "endDate_like")]
    pub end_date_like: Option<String>,
    #[serde(rename = "averageScore")]
    pub average_score: Option<i32>,
    #[serde(rename = "averageScore_not")]
    pub average_score_not: Option<i32>,
    #[serde(rename = "averageScore_greater")]
    pub average_score_greater: Option<i32>,
    #[serde(rename = "averageScore_lesser")]
    pub average_score_lesser: Option<i32>,
    pub popularity: Option<i32>,
    #[serde(rename = "popularity_not")]
    pub popularity_not: Option<i32>,
    #[serde(rename = "popularity_greater")]
    pub popularity_greater: Option<i32>,
    #[serde(rename = "popularity_lesser")]
    pub popularity_lesser: Option<i32>,
    pub episodes: Option<i32>,
    #[serde(rename = "episodes_greater")]
    pub episodes_greater: Option<i32>,
    #[serde(rename = "episodes_lesser")]
    pub episodes_lesser: Option<i32>,
    pub duration: Option<i32>,
    #[serde(rename = "duration_greater")]
    pub duration_greater: Option<i32>,
    #[serde(rename = "duration_lesser")]
    pub duration_lesser: Option<i32>,
    pub chapters: Option<i32>,
    #[serde(rename = "chapters_greater")]
    pub chapters_greater: Option<i32>,
    #[serde(rename = "chapters_lesser")]
    pub chapters_lesser: Option<i32>,
    pub volumes: Option<i32>,
    #[serde(rename = "volumes_greater")]
    pub volumes_greater: Option<i32>,
    #[serde(rename = "volumes_lesser")]
    pub volumes_lesser: Option<i32>,
    pub sort: Option<Vec<MediaSort>>,
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
}

/// Options for fetching detailed information about a single media item.
///
/// This struct allows fetching a single anime/manga with optional pagination
/// for related data like characters, staff, reviews, and recommendations.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FetchMediaOneOptions {
    pub id: Option<i32>,
    // Characters pagination
    #[serde(rename = "fetchCharacters")]
    pub fetch_characters: Option<bool>,
    #[serde(rename = "charactersPage")]
    pub characters_page: Option<i32>,
    #[serde(rename = "charactersPerPage")]
    pub characters_per_page: Option<i32>,
    // Staff pagination
    #[serde(rename = "fetchStaff")]
    pub fetch_staff: Option<bool>,
    #[serde(rename = "staffPage")]
    pub staff_page: Option<i32>,
    #[serde(rename = "staffPerPage")]
    pub staff_per_page: Option<i32>,
    // Reviews pagination
    #[serde(rename = "fetchReviews")]
    pub fetch_reviews: Option<bool>,
    #[serde(rename = "reviewsPage")]
    pub reviews_page: Option<i32>,
    #[serde(rename = "reviewsPerPage")]
    pub reviews_per_page: Option<i32>,
    // Recommendations pagination
    #[serde(rename = "fetchRecommendations")]
    pub fetch_recommendations: Option<bool>,
    #[serde(rename = "recommendationsPage")]
    pub recommendations_page: Option<i32>,
    #[serde(rename = "recommendationsPerPage")]
    pub recommendations_per_page: Option<i32>,
}

/// Endpoint for anime and manga operations.
///
/// This endpoint provides methods for querying anime and manga data
/// from the AniList API, including search, filtering, and detailed
/// information retrieval.
///
/// # Examples
///
/// ```rust
/// # use anilist_moe::AniListClient;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = AniListClient::new();
///
/// // Get trending anime
/// let trending = client.anime().get_trending_anime(Some(1), Some(10)).await?;
///
/// // Search for specific anime
/// let results = client.anime().search_anime("Steins Gate", Some(1), Some(5)).await?;
///
/// // Get anime by ID
/// let anime = client.anime().get_by_id(16498).await?;
/// # Ok(())
/// # }
/// ```
pub struct MediaEndpoint {
    client: AniListClient,
}

impl MediaEndpoint {
    /// Creates a new MediaEndpoint instance.
    pub fn new(client: AniListClient) -> Self {
        Self { client }
    }

    /// Fetches a list of media (anime/manga) with custom options.
    ///
    /// # Arguments
    ///
    /// * `options` - Filtering and pagination options
    ///
    /// # Returns
    ///
    /// Returns `Page<Vec<Media>>` containing the list of media items and pagination info.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use anilist_moe::AniListClient;
    /// # use anilist_moe::endpoints::media::FetchMediaOptions;
    /// # use anilist_moe::enums::media::{MediaType, MediaSort};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AniListClient::new();
    ///
    /// let options = FetchMediaOptions {
    ///     media_type: Some(MediaType::Anime),
    ///     sort: Some(vec![MediaSort::Popularity]),
    ///     page: Some(1),
    ///     per_page: Some(10),
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.anime().fetch(&options).await?;
    /// for anime in &response.data {
    ///     println!("Title: {:?}", anime.title);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch(
        &self,
        options: &FetchMediaOptions,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        let query = media::FETCH;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<Page<Vec<Media>>>, AniListError> =
            self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data),
            Err(err) => Err(err),
        }
    }

    /// Fetches detailed information about a single media item.
    ///
    /// # Arguments
    ///
    /// * `options` - Options including the media ID and optional pagination for related data
    ///
    /// # Returns
    ///
    /// Returns a `Media` object directly (not wrapped in a page structure).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use anilist_moe::AniListClient;
    /// # use anilist_moe::endpoints::media::FetchMediaOneOptions;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AniListClient::new();
    ///
    /// let options = FetchMediaOneOptions {
    ///     id: Some(16498), // Attack on Titan
    ///     ..Default::default()
    /// };
    ///
    /// let anime = client.anime().fetch_one(&options).await?;
    /// println!("Title: {:?}", anime.title);
    /// println!("Score: {:?}", anime.average_score);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_one(&self, options: &FetchMediaOneOptions) -> Result<Media, AniListError> {
        let query = media::FETCH_ONE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<Media>, AniListError> =
            self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data),
            Err(err) => Err(err),
        }
    }

    // Convenience functions - Anime

    /// Get popular anime
    pub async fn get_popular_anime(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(&FetchMediaOptions {
            media_type: Some(MediaType::Anime),
            sort: Some(vec![MediaSort::PopularityDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get trending anime
    pub async fn get_trending_anime(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(&FetchMediaOptions {
            media_type: Some(MediaType::Anime),
            sort: Some(vec![MediaSort::TrendingDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get currently airing anime
    pub async fn get_airing_anime(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(&FetchMediaOptions {
            media_type: Some(MediaType::Anime),
            status: Some(MediaStatus::Releasing),
            sort: Some(vec![MediaSort::PopularityDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get upcoming anime (not yet released)
    pub async fn get_upcoming_anime(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(&FetchMediaOptions {
            media_type: Some(MediaType::Anime),
            status: Some(MediaStatus::NotYetReleased),
            sort: Some(vec![MediaSort::PopularityDesc]),
            page,
            per_page,
            ..Default::default()
        })
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
        self.fetch(&FetchMediaOptions {
            media_type: Some(MediaType::Anime),
            season: Some(season),
            season_year: Some(year),
            sort: Some(vec![MediaSort::PopularityDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Search anime by title
    pub async fn search_anime(
        &self,
        query: &str,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(&FetchMediaOptions {
            media_type: Some(MediaType::Anime),
            search: Some(query.to_string()),
            sort: Some(vec![MediaSort::SearchMatch]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get anime by ID
    pub async fn get_anime_by_id(&self, id: i32) -> Result<Media, AniListError> {
        self.fetch_one(&FetchMediaOneOptions {
            id: Some(id),
            ..Default::default()
        })
        .await
    }

    /// Get top rated anime
    pub async fn get_top_rated_anime(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(&FetchMediaOptions {
            media_type: Some(MediaType::Anime),
            sort: Some(vec![MediaSort::ScoreDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    // Convenience functions - Manga

    /// Get popular manga
    pub async fn get_popular_manga(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(&FetchMediaOptions {
            media_type: Some(MediaType::Manga),
            sort: Some(vec![MediaSort::PopularityDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get trending manga
    pub async fn get_trending_manga(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(&FetchMediaOptions {
            media_type: Some(MediaType::Manga),
            sort: Some(vec![MediaSort::TrendingDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get currently releasing manga
    pub async fn get_releasing_manga(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(&FetchMediaOptions {
            media_type: Some(MediaType::Manga),
            status: Some(MediaStatus::Releasing),
            sort: Some(vec![MediaSort::PopularityDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get completed manga
    pub async fn get_completed_manga(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(&FetchMediaOptions {
            media_type: Some(MediaType::Manga),
            status: Some(MediaStatus::Finished),
            sort: Some(vec![MediaSort::PopularityDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Search manga by title
    pub async fn search_manga(
        &self,
        query: &str,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(&FetchMediaOptions {
            media_type: Some(MediaType::Manga),
            search: Some(query.to_string()),
            sort: Some(vec![MediaSort::SearchMatch]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get manga by ID
    pub async fn get_manga_by_id(&self, id: i32) -> Result<Media, AniListError> {
        self.fetch_one(&FetchMediaOneOptions {
            id: Some(id),
            ..Default::default()
        })
        .await
    }

    /// Get top rated manga
    pub async fn get_top_rated_manga(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Media>>, AniListError> {
        self.fetch(&FetchMediaOptions {
            media_type: Some(MediaType::Manga),
            sort: Some(vec![MediaSort::ScoreDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }
}
