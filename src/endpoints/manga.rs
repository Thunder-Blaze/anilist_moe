use crate::client::AniListClient;
use crate::error::AniListError;
use crate::models::Manga;
use serde_json::json;
use std::collections::HashMap;

pub struct MangaEndpoint {
    client: AniListClient,
}

impl MangaEndpoint {
    pub(crate) fn new(client: AniListClient) -> Self {
        Self { client }
    }

    /// Get popular manga
    pub async fn get_popular(&self, page: i32, per_page: i32) -> Result<Vec<Manga>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    media(type: MANGA, sort: POPULARITY_DESC) {
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
                        chapters
                        volumes
                        genres
                        averageScore
                        meanScore
                        popularity
                        favourites
                        hashtag
                        countryOfOrigin
                        isAdult
                        coverImage {
                            extraLarge
                            large
                            medium
                            color
                        }
                        bannerImage
                        source
                        updatedAt
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
        let manga_list: Vec<Manga> = serde_json::from_value(data)?;
        Ok(manga_list)
    }

    /// Get trending manga
    pub async fn get_trending(&self, page: i32, per_page: i32) -> Result<Vec<Manga>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    media(type: MANGA, sort: TRENDING_DESC) {
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
                        chapters
                        volumes
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
        let manga_list: Vec<Manga> = serde_json::from_value(data)?;
        Ok(manga_list)
    }

    /// Get manga by ID
    pub async fn get_by_id(&self, id: i32) -> Result<Manga, AniListError> {
        let query = r#"
            query ($id: Int) {
                Media(id: $id, type: MANGA) {
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
                    chapters
                    volumes
                    genres
                    averageScore
                    meanScore
                    popularity
                    favourites
                    hashtag
                    countryOfOrigin
                    isAdult
                    coverImage {
                        extraLarge
                        large
                        medium
                        color
                    }
                    bannerImage
                    source
                    updatedAt
                    siteUrl
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("id".to_string(), json!(id));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Media"].clone();
        let manga: Manga = serde_json::from_value(data)?;
        Ok(manga)
    }

    /// Searches for manga by title with advanced filtering and pagination support.
    /// 
    /// Performs a fuzzy search across manga titles in multiple languages (romaji, english, native)
    /// and returns matching results sorted by relevance. The search is case-insensitive and 
    /// supports partial matches. Advanced filtering options allow for more precise searches.
    /// 
    /// # Parameters
    /// 
    /// * `search` - The search query string. Can be partial titles, alternative titles, or keywords.
    ///   Supports searches in romaji, English, and native languages. Optional - if None, returns all results.
    /// * `page` - The page number to retrieve (1-based indexing). Must be positive.
    /// * `per_page` - Number of results to return per page (1-50). Higher values may impact performance.
    /// * `genre` - Filter by specific genre (e.g., "Action", "Romance"). Optional.
    /// * `year` - Filter by release year. Optional.
    /// * `format` - Filter by format ("MANGA", "NOVEL", "ONE_SHOT"). Optional.
    /// 
    /// # Returns
    /// 
    /// Returns a vector of [`Manga`] objects that match the search criteria,
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
    /// let results = client.manga().search_manga(
    ///     Some("One Piece"), 1, 10, None, None, None
    /// ).await?;
    /// 
    /// // Advanced search with filters
    /// let action_manga_2020 = client.manga().search_manga(
    ///     None, 1, 10, Some("Action"), Some(2020), Some("MANGA")
    /// ).await?;
    /// 
    /// // Search for light novels only
    /// let novels = client.manga().search_manga(
    ///     Some("Re:Zero"), 1, 10, None, None, Some("NOVEL")
    /// ).await?;
    /// ```
    /// 
    /// # Available Formats
    /// 
    /// - "MANGA" - Traditional manga
    /// - "NOVEL" - Light novels  
    /// - "ONE_SHOT" - One-shot manga
    /// 
    /// # Note
    /// 
    /// Search results are ranked by AniList's relevance algorithm, which considers
    /// title similarity, popularity, and other factors.
    pub async fn search_manga(
        &self,
        search: Option<&str>,
        page: i32,
        per_page: i32,
        genre: Option<&str>,
        year: Option<i32>,
        format: Option<&str>,
    ) -> Result<Vec<Manga>, AniListError> {
        let query = crate::queries::manga::SEARCH;

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
        if let Some(format_filter) = format {
            variables.insert("format".to_string(), json!(format_filter));
        }

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["media"].clone();
        let manga_list: Vec<Manga> = serde_json::from_value(data)?;
        Ok(manga_list)
    }

    /// Search manga by title (legacy method).
    /// 
    /// This is a simplified version of [`search_manga`] that only supports basic title search.
    /// For advanced filtering capabilities, use [`search_manga`] instead.
    /// 
    /// # Parameters
    /// 
    /// * `search` - The search query string. Can be partial titles, alternative titles, or keywords.
    /// * `page` - The page number to retrieve (1-based indexing). Must be positive.
    /// * `per_page` - Number of results to return per page (1-50). Higher values may impact performance.
    /// 
    /// # Returns
    /// 
    /// Returns a vector of [`Manga`] objects that match the search criteria,
    /// sorted by relevance to the search query.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use anilist_moe::AniListClient;
    /// 
    /// let client = AniListClient::new();
    /// 
    /// // Search for manga with "one piece" in the title
    /// let results = client.manga().search("One Piece", 1, 10).await?;
    /// ```
    pub async fn search(
        &self,
        search: &str,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Manga>, AniListError> {
        self.search_manga(Some(search), page, per_page, None, None, None).await
    }

    /// Get top rated manga
    pub async fn get_top_rated(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Manga>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    media(type: MANGA, sort: SCORE_DESC) {
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
                        chapters
                        volumes
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
        let manga_list: Vec<Manga> = serde_json::from_value(data)?;
        Ok(manga_list)
    }

    /// Get currently releasing manga
    pub async fn get_releasing(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Manga>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    media(type: MANGA, status: RELEASING, sort: POPULARITY_DESC) {
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
                        chapters
                        volumes
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
        let manga_list: Vec<Manga> = serde_json::from_value(data)?;
        Ok(manga_list)
    }

    /// Get completed manga
    pub async fn get_completed(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Manga>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    media(type: MANGA, status: FINISHED, sort: POPULARITY_DESC) {
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
                        chapters
                        volumes
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
        let manga_list: Vec<Manga> = serde_json::from_value(data)?;
        Ok(manga_list)
    }
}
