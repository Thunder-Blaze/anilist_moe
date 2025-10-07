use crate::client::AniListClient;
use crate::error::AniListError;
use crate::models::social::Studio;
use serde_json::json;
use std::collections::HashMap;

pub struct StudioEndpoint {
    client: AniListClient,
}

impl StudioEndpoint {
    pub(crate) fn new(client: AniListClient) -> Self {
        Self { client }
    }

    /// Get popular studios
    pub async fn get_popular(&self, page: i32, per_page: i32) -> Result<Vec<Studio>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    studios(sort: FAVOURITES_DESC) {
                        id
                        name
                        isAnimationStudio
                        siteUrl
                        favourites
                        isFavourite
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["studios"].clone();
        let studios: Vec<Studio> = serde_json::from_value(data)?;
        Ok(studios)
    }

    /// Get studio by ID
    pub async fn get_by_id(&self, id: i32) -> Result<Studio, AniListError> {
        let query = r#"
            query ($id: Int) {
                Studio(id: $id) {
                    id
                    name
                    isAnimationStudio
                    siteUrl
                    favourites
                    isFavourite
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("id".to_string(), json!(id));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Studio"].clone();
        let studio: Studio = serde_json::from_value(data)?;
        Ok(studio)
    }

    /// Search studios by name
    pub async fn search(
        &self,
        search: &str,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Studio>, AniListError> {
        let query = r#"
            query ($search: String, $page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    studios(search: $search) {
                        id
                        name
                        isAnimationStudio
                        siteUrl
                        favourites
                        isFavourite
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("search".to_string(), json!(search));
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["studios"].clone();
        let studios: Vec<Studio> = serde_json::from_value(data)?;
        Ok(studios)
    }

    /// Get most favorited studios
    pub async fn get_most_favorited(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Studio>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    studios(sort: FAVOURITES_DESC) {
                        id
                        name
                        isAnimationStudio
                        siteUrl
                        favourites
                        isFavourite
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["studios"].clone();
        let studios: Vec<Studio> = serde_json::from_value(data)?;
        Ok(studios)
    }

    /// Toggle favorite status of a studio (requires authentication)
    pub async fn toggle_favorite(&self, studio_id: i32) -> Result<Studio, AniListError> {
        let query = r#"
            mutation ($studioId: Int) {
                ToggleFavourite(studioId: $studioId) {
                    studios {
                        nodes {
                            id
                            name
                            isAnimationStudio
                            siteUrl
                            favourites
                            isFavourite
                        }
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("studioId".to_string(), json!(studio_id));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["ToggleFavourite"]["studios"]["nodes"][0].clone();
        let studio: Studio = serde_json::from_value(data)?;
        Ok(studio)
    }
}
