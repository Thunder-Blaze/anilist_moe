//! # Anime Endpoints
//! 
//! This module provides access to all anime-related functionality in the AniList API.
//! It includes methods for searching, browsing, and retrieving detailed information
//! about anime series and movies.

use crate::client::AniListClient;
use crate::error::AniListError;
use crate::models::Anime;
use crate::queries;
use serde_json::json;
use std::collections::HashMap;

/// Endpoint for anime-related API operations.
/// 
/// This struct provides methods to interact with anime data on AniList, including
/// searching, browsing popular/trending content, filtering by season, and retrieving
/// detailed information about specific anime.
/// 
/// All anime endpoints are publicly accessible and do not require authentication.
/// 
/// # Examples
/// 
/// ```rust
/// use anilist_moe::AniListClient;
/// 
/// let client = AniListClient::new();
/// let anime_endpoint = client.anime();
/// 
/// // Search for anime
/// let results = anime_endpoint.search("Attack on Titan", 1, 5).await?;
/// 
/// // Get trending anime
/// let trending = anime_endpoint.get_trending(1, 10).await?;
/// 
/// // Get anime by specific ID
/// let anime = anime_endpoint.get_by_id(16498).await?;
/// ```
pub struct AnimeEndpoint {
    client: AniListClient,
}

impl AnimeEndpoint {
    /// Creates a new anime endpoint instance.
    /// 
    /// This method is typically called internally by [`AniListClient::anime()`]
    /// and should not be used directly.
    /// 
    /// # Parameters
    /// 
    /// * `client` - The AniList client instance to use for API requests
    pub(crate) fn new(client: AniListClient) -> Self {
        Self { client }
    }

    /// Retrieves popular anime with pagination support.
    /// 
    /// Returns a list of anime sorted by popularity in descending order. Popularity
    /// is determined by AniList's algorithm based on user engagement, favorites,
    /// and other metrics.
    /// 
    /// # Parameters
    /// 
    /// * `page` - The page number to retrieve (1-based indexing). Must be positive.
    /// * `per_page` - Number of anime to return per page (1-50). Higher values may impact performance.
    /// 
    /// # Returns
    /// 
    /// Returns a vector of [`Anime`] objects containing comprehensive anime information
    /// including titles, descriptions, ratings, and metadata.
    /// 
    /// # Errors
    /// 
    /// This method can return:
    /// - [`AniListError::BadRequest`] if parameters are invalid (e.g., negative page number)
    /// - [`AniListError::RateLimit`] if rate limits are exceeded
    /// - [`AniListError::Network`] for connection issues
    /// - [`AniListError::Json`] if response parsing fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use anilist_moe::AniListClient;
    /// 
    /// let client = AniListClient::new();
    /// 
    /// // Get the top 10 most popular anime
    /// let popular_anime = client.anime().get_popular(1, 10).await?;
    /// for anime in popular_anime {
    ///     println!("#{} - {} (Score: {})", 
    ///         anime.id,
    ///         anime.title.romaji,
    ///         anime.average_score.unwrap_or(0)
    ///     );
    /// }
    /// 
    /// // Get the next page of popular anime
    /// let more_popular = client.anime().get_popular(2, 10).await?;
    /// ```
    /// 
    /// # Note
    /// 
    /// The popularity ranking is updated regularly by AniList and may change over time.
    /// Results are consistent within short time periods but may vary across longer periods.
    pub async fn get_popular(&self, page: i32, per_page: i32) -> Result<Vec<Anime>, AniListError> {
        let query = queries::anime::GET_POPULAR;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["media"].clone();
        let anime_list: Vec<Anime> = serde_json::from_value(data)?;
        Ok(anime_list)
    }

    /// Retrieves currently trending anime with pagination support.
    /// 
    /// Returns a list of anime that are currently trending on AniList. Trending
    /// is determined by current user activity, recent additions to lists, ratings,
    /// and other real-time engagement metrics.
    /// 
    /// # Parameters
    /// 
    /// * `page` - The page number to retrieve (1-based indexing). Must be positive.
    /// * `per_page` - Number of anime to return per page (1-50). Higher values may impact performance.
    /// 
    /// # Returns
    /// 
    /// Returns a vector of [`Anime`] objects sorted by current trending score,
    /// with the most trending anime first.
    /// 
    /// # Errors
    /// 
    /// This method can return:
    /// - [`AniListError::BadRequest`] if parameters are invalid
    /// - [`AniListError::RateLimit`] if rate limits are exceeded
    /// - [`AniListError::Network`] for connection issues
    /// - [`AniListError::Json`] if response parsing fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use anilist_moe::AniListClient;
    /// 
    /// let client = AniListClient::new();
    /// 
    /// // Get currently trending anime
    /// let trending = client.anime().get_trending(1, 10).await?;
    /// for anime in trending {
    ///     println!("Trending: {} (Popularity: {})", 
    ///         anime.title.romaji,
    ///         anime.popularity.unwrap_or(0)
    ///     );
    /// }
    /// ```
    /// 
    /// # Note
    /// 
    /// Trending data is updated in real-time and can change frequently throughout
    /// the day based on user activity and engagement patterns.
    pub async fn get_trending(&self, page: i32, per_page: i32) -> Result<Vec<Anime>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    media(type: ANIME, sort: TRENDING_DESC) {
                        id
                        title {
                            romaji
                            english
                            native
                            userPreferred
                        }
                        description
                        format
                        status
                        startDate {
                            year
                            month
                            day
                        }
                        endDate {
                            year
                            month
                            day
                        }
                        season
                        seasonYear
                        episodes
                        duration
                        genres
                        averageScore
                        meanScore
                        popularity
                        favourites
                        coverImage {
                            extraLarge
                            large
                            medium
                            color
                        }
                        bannerImage
                        siteUrl
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["media"].clone();
        let anime_list: Vec<Anime> = serde_json::from_value(data)?;
        Ok(anime_list)
    }

    /// Get anime by ID
    pub async fn get_by_id(&self, id: i32) -> Result<Anime, AniListError> {
        let query = r#"
            query ($id: Int) {
                Media(id: $id, type: ANIME) {
                    id
                    title {
                        romaji
                        english
                        native
                        userPreferred
                    }
                    description
                    format
                    status
                    startDate {
                        year
                        month
                        day
                    }
                    endDate {
                        year
                        month
                        day
                    }
                    season
                    seasonYear
                    episodes
                    duration
                    genres
                    averageScore
                    meanScore
                    popularity
                    favourites
                    hashtag
                    countryOfOrigin
                    isAdult
                    nextAiringEpisode {
                        id
                        airingAt
                        timeUntilAiring
                        episode
                        mediaId
                    }
                    coverImage {
                        extraLarge
                        large
                        medium
                        color
                    }
                    bannerImage
                    source
                    trailer {
                        id
                        site
                        thumbnail
                    }
                    updatedAt
                    siteUrl
                    studios {
                        nodes {
                            id
                            name
                            isAnimationStudio
                            siteUrl
                        }
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("id".to_string(), json!(id));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Media"].clone();
        let anime: Anime = serde_json::from_value(data)?;
        Ok(anime)
    }

    /// Searches for anime by title with advanced filtering and pagination support.
    /// 
    /// Performs a fuzzy search across anime titles in multiple languages (romaji, english, native)
    /// and returns matching results sorted by relevance. The search is case-insensitive and 
    /// supports partial matches. Advanced filtering options allow for more precise searches.
    /// 
    /// # Parameters
    /// 
    /// * `search` - The search query string. Can be partial titles, alternative titles, or keywords.
    ///   Supports searches in romaji, English, and native languages. Optional - if None, returns all results.
    /// * `page` - The page number to retrieve (1-based indexing). Must be positive.
    /// * `per_page` - Number of results to return per page (1-50). Higher values may impact performance.
    /// * `genre` - Filter by specific genre (e.g., "Action", "Comedy"). Optional.
    /// * `year` - Filter by release year. Optional.
    /// * `season` - Filter by season ("WINTER", "SPRING", "SUMMER", "FALL"). Optional.
    /// * `format` - Filter by format ("TV", "MOVIE", "OVA", "ONA", "SPECIAL", "MUSIC"). Optional.
    /// 
    /// # Returns
    /// 
    /// Returns a vector of [`Anime`] objects that match the search criteria,
    /// sorted by relevance to the search query.
    /// 
    /// # Errors
    /// 
    /// This method can return:
    /// - [`AniListError::BadRequest`] if parameters are invalid
    /// - [`AniListError::RateLimit`] if rate limits are exceeded  
    /// - [`AniListError::Network`] for connection issues
    /// - [`AniListError::Json`] if response parsing fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use anilist_moe::AniListClient;
    /// 
    /// let client = AniListClient::new();
    /// 
    /// // Basic search
    /// let results = client.anime().search_anime(
    ///     Some("Attack on Titan"), 1, 10, None, None, None, None
    /// ).await?;
    /// 
    /// // Advanced search with filters
    /// let action_anime_2023 = client.anime().search_anime(
    ///     None, 1, 10, Some("Action"), Some(2023), Some("FALL"), Some("TV")
    /// ).await?;
    /// 
    /// // Search for movies only
    /// let movies = client.anime().search_anime(
    ///     Some("Studio Ghibli"), 1, 10, None, None, None, Some("MOVIE")
    /// ).await?;
    /// ```
    /// 
    /// # Search Tips
    /// 
    /// - Use specific keywords for better results
    /// - Combine filters for more precise searches
    /// - Try alternative titles if initial search doesn't yield expected results
    /// - Search supports both romaji and native script titles
    /// - Partial matches are supported (e.g., "attack" will match "Attack on Titan")
    /// 
    /// # Available Formats
    /// 
    /// - "TV" - Television series
    /// - "MOVIE" - Theatrical films
    /// - "OVA" - Original Video Animation
    /// - "ONA" - Original Net Animation
    /// - "SPECIAL" - Special episodes
    /// - "MUSIC" - Music videos
    /// 
    /// # Available Seasons
    /// 
    /// - "WINTER" - January-March
    /// - "SPRING" - April-June  
    /// - "SUMMER" - July-September
    /// - "FALL" - October-December
    /// 
    /// # Note
    /// 
    /// Search results are ranked by AniList's relevance algorithm, which considers
    /// title similarity, popularity, and other factors.
    pub async fn search_anime(
        &self,
        search: Option<&str>,
        page: i32,
        per_page: i32,
        genre: Option<&str>,
        year: Option<i32>,
        season: Option<&str>,
        format: Option<&str>,
    ) -> Result<Vec<Anime>, AniListError> {
        let query = queries::anime::SEARCH;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));
        
        if let Some(search_term) = search {
            variables.insert("search".to_string(), json!(search_term));
        }
        if let Some(genre_filter) = genre {
            variables.insert("genre".to_string(), json!(genre_filter));
        }
        if let Some(year_filter) = year {
            variables.insert("year".to_string(), json!(year_filter));
        }
        if let Some(season_filter) = season {
            variables.insert("season".to_string(), json!(season_filter));
        }
        if let Some(format_filter) = format {
            variables.insert("format".to_string(), json!(format_filter));
        }

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["media"].clone();
        let anime_list: Vec<Anime> = serde_json::from_value(data)?;
        Ok(anime_list)
    }

    /// Searches for anime by title with pagination support (legacy method).
    /// 
    /// This is a simplified version of [`search_anime`] that only supports basic title search.
    /// For advanced filtering capabilities, use [`search_anime`] instead.
    /// 
    /// # Parameters
    /// 
    /// * `search` - The search query string. Can be partial titles, alternative titles, or keywords.
    /// * `page` - The page number to retrieve (1-based indexing). Must be positive.
    /// * `per_page` - Number of results to return per page (1-50). Higher values may impact performance.
    /// 
    /// # Returns
    /// 
    /// Returns a vector of [`Anime`] objects that match the search criteria,
    /// sorted by relevance to the search query.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use anilist_moe::AniListClient;
    /// 
    /// let client = AniListClient::new();
    /// 
    /// // Search for anime with "attack" in the title
    /// let results = client.anime().search("attack", 1, 10).await?;
    /// ```
    pub async fn search(
        &self,
        search: &str,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Anime>, AniListError> {
        self.search_anime(Some(search), page, per_page, None, None, None, None).await
    }

    /// Get anime by season and year
    pub async fn get_by_season(
        &self,
        season: &str,
        year: i32,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Anime>, AniListError> {
        let query = r#"
            query ($season: MediaSeason, $year: Int, $page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    media(type: ANIME, season: $season, seasonYear: $year, sort: POPULARITY_DESC) {
                        id
                        title {
                            romaji
                            english
                            native
                            userPreferred
                        }
                        description
                        format
                        status
                        season
                        seasonYear
                        episodes
                        duration
                        genres
                        averageScore
                        meanScore
                        popularity
                        favourites
                        coverImage {
                            extraLarge
                            large
                            medium
                            color
                        }
                        bannerImage
                        siteUrl
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("season".to_string(), json!(season.to_uppercase()));
        variables.insert("year".to_string(), json!(year));
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["media"].clone();
        let anime_list: Vec<Anime> = serde_json::from_value(data)?;
        Ok(anime_list)
    }

    /// Get top rated anime
    pub async fn get_top_rated(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Anime>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    media(type: ANIME, sort: SCORE_DESC) {
                        id
                        title {
                            romaji
                            english
                            native
                            userPreferred
                        }
                        description
                        format
                        status
                        season
                        seasonYear
                        episodes
                        duration
                        genres
                        averageScore
                        meanScore
                        popularity
                        favourites
                        coverImage {
                            extraLarge
                            large
                            medium
                            color
                        }
                        bannerImage
                        siteUrl
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["media"].clone();
        let anime_list: Vec<Anime> = serde_json::from_value(data)?;
        Ok(anime_list)
    }

    /// Get currently airing anime
    pub async fn get_airing(&self, page: i32, per_page: i32) -> Result<Vec<Anime>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    media(type: ANIME, status: RELEASING, sort: POPULARITY_DESC) {
                        id
                        title {
                            romaji
                            english
                            native
                            userPreferred
                        }
                        description
                        format
                        status
                        season
                        seasonYear
                        episodes
                        duration
                        genres
                        averageScore
                        meanScore
                        popularity
                        favourites
                        nextAiringEpisode {
                            id
                            airingAt
                            timeUntilAiring
                            episode
                            mediaId
                        }
                        coverImage {
                            extraLarge
                            large
                            medium
                            color
                        }
                        bannerImage
                        siteUrl
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["media"].clone();
        let anime_list: Vec<Anime> = serde_json::from_value(data)?;
        Ok(anime_list)
    }
}
