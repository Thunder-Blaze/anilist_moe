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

pub struct AnimeEndpoint {
    client: AniListClient,
}

impl AnimeEndpoint {
    pub(crate) fn new(client: AniListClient) -> Self {
        Self { client }
    }

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

    pub async fn search(
        &self,
        search: &str,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Anime>, AniListError> {
        self.search_anime(Some(search), page, per_page, None, None, None, None).await
    }

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
