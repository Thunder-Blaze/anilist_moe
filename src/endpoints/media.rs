use crate::enums::media::{MediaFormat, MediaSeason, MediaSort, MediaStatus, MediaType};
use crate::errors::AniListError;
use crate::objects::responses::{MediaListResponse, MediaSingleResponse};
use crate::{client::AniListClient, queries::media};
use serde::Serialize;
use serde_json::json;

/// Options for fetching media (anime/manga) with various filters.
#[derive(Default, Serialize)]
pub struct FetchMediaOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "idMal", skip_serializing_if = "Option::is_none")]
    pub id_mal: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<MediaType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<MediaFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MediaStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<MediaSeason>,
    #[serde(rename = "seasonYear", skip_serializing_if = "Option::is_none")]
    pub season_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "tagCategory", skip_serializing_if = "Option::is_none")]
    pub tag_category: Option<String>,
    #[serde(rename = "minimumTagRank", skip_serializing_if = "Option::is_none")]
    pub minimum_tag_rank: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "countryOfOrigin", skip_serializing_if = "Option::is_none")]
    pub country_of_origin: Option<String>,
    #[serde(rename = "isLicensed", skip_serializing_if = "Option::is_none")]
    pub is_licensed: Option<bool>,
    #[serde(rename = "isAdult", skip_serializing_if = "Option::is_none")]
    pub is_adult: Option<bool>,
    #[serde(rename = "onList", skip_serializing_if = "Option::is_none")]
    pub on_list: Option<bool>,
    #[serde(rename = "licensedBy", skip_serializing_if = "Option::is_none")]
    pub licensed_by: Option<String>,
    #[serde(rename = "licensedById", skip_serializing_if = "Option::is_none")]
    pub licensed_by_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_not: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_not_in: Option<Vec<i32>>,
    #[serde(rename = "idMal_not", skip_serializing_if = "Option::is_none")]
    pub id_mal_not: Option<i32>,
    #[serde(rename = "idMal_in", skip_serializing_if = "Option::is_none")]
    pub id_mal_in: Option<Vec<i32>>,
    #[serde(rename = "idMal_not_in", skip_serializing_if = "Option::is_none")]
    pub id_mal_not_in: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_not: Option<MediaFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_in: Option<Vec<MediaFormat>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_not_in: Option<Vec<MediaFormat>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_not: Option<MediaStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_in: Option<Vec<MediaStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_not_in: Option<Vec<MediaStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre_in: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre_not_in: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_in: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_not_in: Option<Vec<String>>,
    #[serde(rename = "tagCategory_in", skip_serializing_if = "Option::is_none")]
    pub tag_category_in: Option<Vec<String>>,
    #[serde(rename = "tagCategory_not_in", skip_serializing_if = "Option::is_none")]
    pub tag_category_not_in: Option<Vec<String>>,
    #[serde(rename = "licensedBy_in", skip_serializing_if = "Option::is_none")]
    pub licensed_by_in: Option<Vec<String>>,
    #[serde(rename = "licensedById_in", skip_serializing_if = "Option::is_none")]
    pub licensed_by_id_in: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_in: Option<Vec<String>>,
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<i32>,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<i32>,
    #[serde(rename = "startDate_greater", skip_serializing_if = "Option::is_none")]
    pub start_date_greater: Option<i32>,
    #[serde(rename = "startDate_lesser", skip_serializing_if = "Option::is_none")]
    pub start_date_lesser: Option<i32>,
    #[serde(rename = "startDate_like", skip_serializing_if = "Option::is_none")]
    pub start_date_like: Option<String>,
    #[serde(rename = "endDate_greater", skip_serializing_if = "Option::is_none")]
    pub end_date_greater: Option<i32>,
    #[serde(rename = "endDate_lesser", skip_serializing_if = "Option::is_none")]
    pub end_date_lesser: Option<i32>,
    #[serde(rename = "endDate_like", skip_serializing_if = "Option::is_none")]
    pub end_date_like: Option<String>,
    #[serde(rename = "averageScore", skip_serializing_if = "Option::is_none")]
    pub average_score: Option<i32>,
    #[serde(rename = "averageScore_not", skip_serializing_if = "Option::is_none")]
    pub average_score_not: Option<i32>,
    #[serde(
        rename = "averageScore_greater",
        skip_serializing_if = "Option::is_none"
    )]
    pub average_score_greater: Option<i32>,
    #[serde(
        rename = "averageScore_lesser",
        skip_serializing_if = "Option::is_none"
    )]
    pub average_score_lesser: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub popularity: Option<i32>,
    #[serde(rename = "popularity_not", skip_serializing_if = "Option::is_none")]
    pub popularity_not: Option<i32>,
    #[serde(rename = "popularity_greater", skip_serializing_if = "Option::is_none")]
    pub popularity_greater: Option<i32>,
    #[serde(rename = "popularity_lesser", skip_serializing_if = "Option::is_none")]
    pub popularity_lesser: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episodes: Option<i32>,
    #[serde(rename = "episodes_greater", skip_serializing_if = "Option::is_none")]
    pub episodes_greater: Option<i32>,
    #[serde(rename = "episodes_lesser", skip_serializing_if = "Option::is_none")]
    pub episodes_lesser: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "duration_greater", skip_serializing_if = "Option::is_none")]
    pub duration_greater: Option<i32>,
    #[serde(rename = "duration_lesser", skip_serializing_if = "Option::is_none")]
    pub duration_lesser: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapters: Option<i32>,
    #[serde(rename = "chapters_greater", skip_serializing_if = "Option::is_none")]
    pub chapters_greater: Option<i32>,
    #[serde(rename = "chapters_lesser", skip_serializing_if = "Option::is_none")]
    pub chapters_lesser: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<i32>,
    #[serde(rename = "volumes_greater", skip_serializing_if = "Option::is_none")]
    pub volumes_greater: Option<i32>,
    #[serde(rename = "volumes_lesser", skip_serializing_if = "Option::is_none")]
    pub volumes_lesser: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<MediaSort>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
}

#[derive(Default, Serialize)]
pub struct FetchMediaOneOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    // Characters pagination
    #[serde(rename = "fetchCharacters", skip_serializing_if = "Option::is_none")]
    pub fetch_characters: Option<bool>,
    #[serde(rename = "charactersPage", skip_serializing_if = "Option::is_none")]
    pub characters_page: Option<i32>,
    #[serde(rename = "charactersPerPage", skip_serializing_if = "Option::is_none")]
    pub characters_per_page: Option<i32>,
    // Staff pagination
    #[serde(rename = "fetchStaff", skip_serializing_if = "Option::is_none")]
    pub fetch_staff: Option<bool>,
    #[serde(rename = "staffPage", skip_serializing_if = "Option::is_none")]
    pub staff_page: Option<i32>,
    #[serde(rename = "staffPerPage", skip_serializing_if = "Option::is_none")]
    pub staff_per_page: Option<i32>,
    // Reviews pagination
    #[serde(rename = "fetchReviews", skip_serializing_if = "Option::is_none")]
    pub fetch_reviews: Option<bool>,
    #[serde(rename = "reviewsPage", skip_serializing_if = "Option::is_none")]
    pub reviews_page: Option<i32>,
    #[serde(rename = "reviewsPerPage", skip_serializing_if = "Option::is_none")]
    pub reviews_per_page: Option<i32>,
    // Recommendations pagination
    #[serde(
        rename = "fetchRecommendations",
        skip_serializing_if = "Option::is_none"
    )]
    pub fetch_recommendations: Option<bool>,
    #[serde(
        rename = "recommendationsPage",
        skip_serializing_if = "Option::is_none"
    )]
    pub recommendations_page: Option<i32>,
    #[serde(
        rename = "recommendationsPerPage",
        skip_serializing_if = "Option::is_none"
    )]
    pub recommendations_per_page: Option<i32>,
}

/// Endpoint for anime and manga operations.
pub struct MediaEndpoint {
    client: AniListClient,
}

impl MediaEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: FetchMediaOptions,
    ) -> Result<MediaListResponse, AniListError> {
        let query = media::FETCH;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn fetch_one(
        &self,
        options: FetchMediaOneOptions,
    ) -> Result<MediaSingleResponse, AniListError> {
        let query = media::FETCH_ONE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    // Convenience functions - Anime

    /// Get popular anime
    pub async fn get_popular_anime(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<MediaListResponse, AniListError> {
        self.fetch(FetchMediaOptions {
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
    ) -> Result<MediaListResponse, AniListError> {
        self.fetch(FetchMediaOptions {
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
    ) -> Result<MediaListResponse, AniListError> {
        self.fetch(FetchMediaOptions {
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
    ) -> Result<MediaListResponse, AniListError> {
        self.fetch(FetchMediaOptions {
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
    ) -> Result<MediaListResponse, AniListError> {
        self.fetch(FetchMediaOptions {
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
    ) -> Result<MediaListResponse, AniListError> {
        self.fetch(FetchMediaOptions {
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
    pub async fn get_anime_by_id(&self, id: i32) -> Result<MediaSingleResponse, AniListError> {
        self.fetch_one(FetchMediaOneOptions {
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
    ) -> Result<MediaListResponse, AniListError> {
        self.fetch(FetchMediaOptions {
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
    ) -> Result<MediaListResponse, AniListError> {
        self.fetch(FetchMediaOptions {
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
    ) -> Result<MediaListResponse, AniListError> {
        self.fetch(FetchMediaOptions {
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
    ) -> Result<MediaListResponse, AniListError> {
        self.fetch(FetchMediaOptions {
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
    ) -> Result<MediaListResponse, AniListError> {
        self.fetch(FetchMediaOptions {
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
    ) -> Result<MediaListResponse, AniListError> {
        self.fetch(FetchMediaOptions {
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
    pub async fn get_manga_by_id(&self, id: i32) -> Result<MediaSingleResponse, AniListError> {
        self.fetch_one(FetchMediaOneOptions {
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
    ) -> Result<MediaListResponse, AniListError> {
        self.fetch(FetchMediaOptions {
            media_type: Some(MediaType::Manga),
            sort: Some(vec![MediaSort::ScoreDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }
}
