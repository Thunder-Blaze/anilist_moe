use crate::client::AniListClient;
use crate::errors::AniListError;
use crate::helpers::query_builders::{QueryBuilder, QueryType, MediaSearchQueryBuilder};
use crate::enums::media::{MediaFormat, MediaSeason, MediaSort, MediaStatus, MediaType};
use crate::objects::responses::MediaListResponse;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Default, Serialize, Deserialize)]
pub struct AnimeSearchOptions {
    // Basic search
    pub search_term: Option<String>,
    pub id: Option<i32>,
    pub format: Option<Vec<MediaFormat>>,
    pub status: Option<MediaStatus>,
    pub season: Option<MediaSeason>,
    pub season_year: Option<i32>,
    pub year: Option<String>,
    pub genre: Option<Vec<String>>,
    pub tag: Option<Vec<String>>,

    // Extended parameters
    pub id_mal: Option<i32>,
    pub start_date: Option<i32>,
    pub end_date: Option<i32>,
    pub episodes: Option<i32>,
    pub duration: Option<i32>,
    pub chapters: Option<i32>,
    pub volumes: Option<i32>,
    pub is_adult: Option<bool>,
    pub is_licensed: Option<bool>,
    pub average_score: Option<i32>,
    pub popularity: Option<i32>,
    pub source: Option<String>,
    pub country_of_origin: Option<String>,

    // NOT filters
    pub id_not: Option<i32>,
    pub id_in: Option<Vec<i32>>,
    pub id_not_in: Option<Vec<i32>>,
    pub id_mal_not: Option<i32>,
    pub id_mal_in: Option<Vec<i32>>,
    pub id_mal_not_in: Option<Vec<i32>>,
    pub format_not: Option<MediaFormat>,
    pub format_in: Option<Vec<MediaFormat>>,
    pub format_not_in: Option<Vec<MediaFormat>>,
    pub status_not: Option<MediaStatus>,
    pub status_in: Option<Vec<MediaStatus>>,
    pub status_not_in: Option<Vec<MediaStatus>>,
    pub genre_not_in: Option<Vec<String>>,
    pub tag_not_in: Option<Vec<String>>,

    // Range filters
    pub episodes_greater: Option<i32>,
    pub episodes_lesser: Option<i32>,
    pub duration_greater: Option<i32>,
    pub duration_lesser: Option<i32>,
    pub chapters_greater: Option<i32>,
    pub chapters_lesser: Option<i32>,
    pub volumes_greater: Option<i32>,
    pub volumes_lesser: Option<i32>,
    pub average_score_greater: Option<i32>,
    pub average_score_lesser: Option<i32>,
    pub popularity_greater: Option<i32>,
    pub popularity_lesser: Option<i32>,
    pub start_date_greater: Option<i32>,
    pub start_date_lesser: Option<i32>,
    pub start_date_like: Option<String>,
    pub end_date_greater: Option<i32>,
    pub end_date_lesser: Option<i32>,
    pub end_date_like: Option<String>,

    // Array filters
    pub genre_in: Option<Vec<String>>,
    pub tag_in: Option<Vec<String>>,

    pub sort: Option<Vec<MediaSort>>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct MangaSearchOptions {
    // Basic search
    pub search_term: Option<String>,
    pub id: Option<i32>,
    pub format: Option<Vec<MediaFormat>>,
    pub status: Option<MediaStatus>,
    pub year: Option<String>,
    pub genre: Option<Vec<String>>,
    pub tag: Option<Vec<String>>,

    // Extended parameters
    pub id_mal: Option<i32>,
    pub start_date: Option<i32>,
    pub end_date: Option<i32>,
    pub episodes: Option<i32>,
    pub duration: Option<i32>,
    pub chapters: Option<i32>,
    pub volumes: Option<i32>,
    pub is_adult: Option<bool>,
    pub is_licensed: Option<bool>,
    pub average_score: Option<i32>,
    pub popularity: Option<i32>,
    pub source: Option<String>,
    pub country_of_origin: Option<String>,

    // NOT filters
    pub id_not: Option<i32>,
    pub id_in: Option<Vec<i32>>,
    pub id_not_in: Option<Vec<i32>>,
    pub id_mal_not: Option<i32>,
    pub id_mal_in: Option<Vec<i32>>,
    pub id_mal_not_in: Option<Vec<i32>>,
    pub format_not: Option<MediaFormat>,
    pub format_in: Option<Vec<MediaFormat>>,
    pub format_not_in: Option<Vec<MediaFormat>>,
    pub status_not: Option<MediaStatus>,
    pub status_in: Option<Vec<MediaStatus>>,
    pub status_not_in: Option<Vec<MediaStatus>>,
    pub genre_not_in: Option<Vec<String>>,
    pub tag_not_in: Option<Vec<String>>,

    // Range filters
    pub episodes_greater: Option<i32>,
    pub episodes_lesser: Option<i32>,
    pub duration_greater: Option<i32>,
    pub duration_lesser: Option<i32>,
    pub chapters_greater: Option<i32>,
    pub chapters_lesser: Option<i32>,
    pub volumes_greater: Option<i32>,
    pub volumes_lesser: Option<i32>,
    pub average_score_greater: Option<i32>,
    pub average_score_lesser: Option<i32>,
    pub popularity_greater: Option<i32>,
    pub popularity_lesser: Option<i32>,
    pub start_date_greater: Option<i32>,
    pub start_date_lesser: Option<i32>,
    pub start_date_like: Option<String>,
    pub end_date_greater: Option<i32>,
    pub end_date_lesser: Option<i32>,
    pub end_date_like: Option<String>,

    // Array filters
    pub genre_in: Option<Vec<String>>,
    pub tag_in: Option<Vec<String>>,

    pub sort: Option<Vec<MediaSort>>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

pub struct MediaEndpoint {
    client: AniListClient,
}

impl MediaEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self { client }
    }

    /// General search method - allows full customization using QueryBuilder (only MediaSearch allowed)
    /// Usage: Pass a QueryBuilder created with QueryType::MediaSearch
    pub async fn search(&self, query_builder: MediaSearchQueryBuilder) -> Result<MediaListResponse, AniListError> {
        // Note: QueryBuilder must be created with QueryType::MediaSearch
        let variables = query_builder.build();
        let variables_map = self.value_to_hashmap(variables);

        let query = include_str!("../queries/media/search.graphql");
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Get popular anime (specific endpoint)
    pub async fn get_popular_anime(&self, page: Option<i32>, per_page: Option<i32>) -> Result<MediaListResponse, AniListError> {
        let mut query_builder = MediaSearchQueryBuilder::new(QueryType::MediaSearch);
        query_builder = query_builder.media_type(Some(MediaType::Anime))
            .sort_media(Some(vec![MediaSort::PopularityDesc]));

        if let Some(p) = page {
            query_builder = query_builder.page(Some(p));
        }
        if let Some(pp) = per_page {
            query_builder = query_builder.per_page(Some(pp));
        }

        self.search(query_builder).await
    }

    /// Get trending anime (specific endpoint)
    pub async fn get_trending_anime(&self, page: Option<i32>, per_page: Option<i32>) -> Result<MediaListResponse, AniListError> {
        let mut query_builder = MediaSearchQueryBuilder::new(QueryType::MediaSearch);
        query_builder = query_builder.media_type(Some(MediaType::Anime))
            .sort_media(Some(vec![MediaSort::TrendingDesc]));

        if let Some(p) = page {
            query_builder = query_builder.page(Some(p));
        }
        if let Some(pp) = per_page {
            query_builder = query_builder.per_page(Some(pp));
        }

        self.search(query_builder).await
    }

    /// Get top rated anime (specific endpoint)
    pub async fn get_top_rated_anime(&self, page: Option<i32>, per_page: Option<i32>) -> Result<Value, AniListError> {
        let variables = QueryBuilder::new(QueryType::MediaSearch)
            .media_type(Some(MediaType::Anime))
            .sort_media(Some(vec![MediaSort::ScoreDesc]))
            .page(page)
            .per_page(per_page)
            .build();

        let variables_map = self.value_to_hashmap(variables);
        let query = include_str!("../queries/media/search.graphql");
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Get anime by season (specific endpoint)
    pub async fn get_anime_by_season(
        &self,
        season: MediaSeason,
        year: i32,
        page: Option<i32>,
        per_page: Option<i32>
    ) -> Result<Value, AniListError> {
        let variables = QueryBuilder::new(QueryType::MediaSearch)
            .media_type(Some(MediaType::Anime))
            .season(Some(season))
            .season_year(Some(year))
            .sort_media(Some(vec![MediaSort::PopularityDesc]))
            .page(page)
            .per_page(per_page)
            .build();

        let variables_map = self.value_to_hashmap(variables);
        let query = include_str!("../queries/media/search.graphql");
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Get currently airing anime (specific endpoint)
    pub async fn get_airing_anime(&self, page: Option<i32>, per_page: Option<i32>) -> Result<Value, AniListError> {
        let variables = QueryBuilder::new(QueryType::MediaSearch)
            .media_type(Some(MediaType::Anime))
            .status(Some(MediaStatus::Releasing))
            .sort_media(Some(vec![MediaSort::PopularityDesc]))
            .page(page)
            .per_page(per_page)
            .build();

        let variables_map = self.value_to_hashmap(variables);
        let query = include_str!("../queries/media/search.graphql");
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Get anime by format (TV, Movie, etc.) (specific endpoint)
    pub async fn get_anime_by_format(
        &self,
        format: Vec<MediaFormat>,
        page: Option<i32>,
        per_page: Option<i32>
    ) -> Result<Value, AniListError> {
        let variables = QueryBuilder::new(QueryType::MediaSearch)
            .media_type(Some(MediaType::Anime))
            .format(Some(format))
            .sort_media(Some(vec![MediaSort::PopularityDesc]))
            .page(page)
            .per_page(per_page)
            .build();

        let variables_map = self.value_to_hashmap(variables);
        let query = include_str!("../queries/media/search.graphql");
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Get anime by genre (specific endpoint)
    pub async fn get_anime_by_genre(
        &self,
        genres: Vec<String>,
        page: Option<i32>,
        per_page: Option<i32>
    ) -> Result<Value, AniListError> {
        let variables = QueryBuilder::new(QueryType::MediaSearch)
            .media_type(Some(MediaType::Anime))
            .genre(Some(genres))
            .sort_media(Some(vec![MediaSort::PopularityDesc]))
            .page(page)
            .per_page(per_page)
            .build();

        let variables_map = self.value_to_hashmap(variables);
        let query = include_str!("../queries/media/search.graphql");
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Get anime by ID (specific endpoint)
    pub async fn get_anime_by_id(&self, id: i32) -> Result<MediaListResponse, AniListError> {
        let variables = QueryBuilder::new(QueryType::MediaSearch)
            .id(Some(id))
            .media_type(Some(MediaType::Anime))
            .extended(Some(true)) // Get extended data for single anime
            .build();

        let variables_map = self.value_to_hashmap(variables);
        let query = include_str!("../queries/media/search.graphql");
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Search anime with all search options (media type set to Anime internally)
    pub async fn search_anime(&self, options: AnimeSearchOptions) -> Result<MediaListResponse, AniListError> {
        let variables = QueryBuilder::new(QueryType::MediaSearch)
            .search(options.search_term)
            .media_type(Some(MediaType::Anime)) // Set internally
            .format(options.format)
            .status(options.status)
            .season(options.season)
            .season_year(options.season_year)
            .year(options.year)
            .genre(options.genre)
            .tag(options.tag)
            .sort_media(options.sort)
            .page(options.page)
            .per_page(options.per_page)
            .build();

        let variables_map = self.value_to_hashmap(variables);
        let query = include_str!("../queries/media/search.graphql");
        self.client.query_typed(query, Some(&variables_map)).await
    }

    // === MANGA METHODS ===

    /// Get popular manga (specific endpoint)
    pub async fn get_popular_manga(&self, page: Option<i32>, per_page: Option<i32>) -> Result<MediaListResponse, AniListError> {
        let variables = QueryBuilder::new(QueryType::MediaSearch)
            .media_type(Some(MediaType::Manga))
            .sort_media(Some(vec![MediaSort::PopularityDesc]))
            .page(page)
            .per_page(per_page)
            .build();

        let variables_map = self.value_to_hashmap(variables);
        let query = include_str!("../queries/media/search.graphql");
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Get trending manga (specific endpoint)
    pub async fn get_trending_manga(&self, page: Option<i32>, per_page: Option<i32>) -> Result<MediaListResponse, AniListError> {
        let variables = QueryBuilder::new(QueryType::MediaSearch)
            .media_type(Some(MediaType::Manga))
            .sort_media(Some(vec![MediaSort::TrendingDesc]))
            .page(page)
            .per_page(per_page)
            .build();

        let variables_map = self.value_to_hashmap(variables);
        let query = include_str!("../queries/media/search.graphql");
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Get top rated manga (specific endpoint)
    pub async fn get_top_rated_manga(&self, page: Option<i32>, per_page: Option<i32>) -> Result<MediaListResponse, AniListError> {
        let variables = QueryBuilder::new(QueryType::MediaSearch)
            .media_type(Some(MediaType::Manga))
            .sort_media(Some(vec![MediaSort::ScoreDesc]))
            .page(page)
            .per_page(per_page)
            .build();

        let variables_map = self.value_to_hashmap(variables);
        let query = include_str!("../queries/media/search.graphql");
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Get currently releasing manga (specific endpoint)
    pub async fn get_releasing_manga(&self, page: Option<i32>, per_page: Option<i32>) -> Result<Value, AniListError> {
        let variables = QueryBuilder::new(QueryType::MediaSearch)
            .media_type(Some(MediaType::Manga))
            .status(Some(MediaStatus::Releasing))
            .sort_media(Some(vec![MediaSort::PopularityDesc]))
            .page(page)
            .per_page(per_page)
            .build();

        let variables_map = self.value_to_hashmap(variables);
        let query = include_str!("../queries/media/search.graphql");
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Get manga by ID (specific endpoint)
    pub async fn get_manga_by_id(&self, id: i32) -> Result<Value, AniListError> {
        let variables = QueryBuilder::new(QueryType::MediaSearch)
            .id(Some(id))
            .media_type(Some(MediaType::Manga))
            .extended(Some(true)) // Get extended data for single manga
            .build();

        let variables_map = self.value_to_hashmap(variables);
        let query = include_str!("../queries/media/search.graphql");
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Search manga with all search options (media type set to Manga internally)
    pub async fn search_manga(&self, options: MangaSearchOptions) -> Result<Value, AniListError> {
        let variables = QueryBuilder::new(QueryType::MediaSearch)
            .search(options.search_term)
            .media_type(Some(MediaType::Manga)) // Set internally
            .format(options.format)
            .status(options.status)
            .year(options.year)
            .genre(options.genre)
            .tag(options.tag)
            .sort_media(options.sort)
            .page(options.page)
            .per_page(options.per_page)
            .build();

        let variables_map = self.value_to_hashmap(variables);
        let query = include_str!("../queries/media/search.graphql");
        self.client.query_typed(query, Some(&variables_map)).await
    }

    // Helper method to convert Value to HashMap<String, Value>
    fn value_to_hashmap(&self, value: Value) -> HashMap<String, Value> {
        match value {
            Value::Object(map) => map.into_iter().collect(),
            _ => HashMap::new(),
        }
    }
}
