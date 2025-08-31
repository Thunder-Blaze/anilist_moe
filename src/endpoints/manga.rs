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

    pub async fn search(
        &self,
        search: &str,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Manga>, AniListError> {
        self.search_manga(Some(search), page, per_page, None, None, None).await
    }

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
