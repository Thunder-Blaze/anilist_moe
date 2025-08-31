use crate::client::AniListClient;
use crate::error::AniListError;
use crate::models::character::Character;
use serde_json::json;
use std::collections::HashMap;

pub struct CharacterEndpoint {
    client: AniListClient,
}

impl CharacterEndpoint {
    pub(crate) fn new(client: AniListClient) -> Self {
        Self { client }
    }

    pub async fn get_popular(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Character>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    characters(sort: FAVOURITES_DESC) {
                        id
                        name {
                            first
                            middle
                            last
                            full
                            native
                            alternative
                            alternativeSpoiler
                            userPreferred
                        }
                        image {
                            large
                            medium
                        }
                        description
                        gender
                        dateOfBirth {
                            year
                            month
                            day
                        }
                        age
                        bloodType
                        isFavourite
                        isFavouriteBlocked
                        siteUrl
                        favourites
                        modNotes
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["characters"].clone();
        let characters: Vec<Character> = serde_json::from_value(data)?;
        Ok(characters)
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Character, AniListError> {
        let query = r#"
            query ($id: Int) {
                Character(id: $id) {
                    id
                    name {
                        first
                        middle
                        last
                        full
                        native
                        alternative
                        alternativeSpoiler
                        userPreferred
                    }
                    image {
                        large
                        medium
                    }
                    description
                    gender
                    dateOfBirth {
                        year
                        month
                        day
                    }
                    age
                    bloodType
                    isFavourite
                    isFavouriteBlocked
                    siteUrl
                    favourites
                    modNotes
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("id".to_string(), json!(id));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Character"].clone();
        let character: Character = serde_json::from_value(data)?;
        Ok(character)
    }

    pub async fn search(
        &self,
        search: &str,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Character>, AniListError> {
        let query = r#"
            query ($search: String, $page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    characters(search: $search) {
                        id
                        name {
                            first
                            middle
                            last
                            full
                            native
                            alternative
                            alternativeSpoiler
                            userPreferred
                        }
                        image {
                            large
                            medium
                        }
                        description
                        gender
                        dateOfBirth {
                            year
                            month
                            day
                        }
                        age
                        bloodType
                        isFavourite
                        siteUrl
                        favourites
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("search".to_string(), json!(search));
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["characters"].clone();
        let characters: Vec<Character> = serde_json::from_value(data)?;
        Ok(characters)
    }

    pub async fn get_today_birthday(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Character>, AniListError> {
        let query: &'static str = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    characters(sort: FAVOURITES_DESC, isBirthday: true) {
                        id
                        name {
                            first
                            middle
                            last
                            full
                            native
                            alternative
                            userPreferred
                        }
                        image {
                            large
                            medium
                        }
                        description
                        gender
                        dateOfBirth {
                            year
                            month
                            day
                        }
                        age
                        bloodType
                        siteUrl
                        favourites
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["characters"].clone();
        let characters: Vec<Character> = serde_json::from_value(data)?;

        Ok(characters)
    }

    pub async fn get_most_favorited(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Character>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    characters(sort: FAVOURITES_DESC) {
                        id
                        name {
                            first
                            middle
                            last
                            full
                            native
                            alternative
                            userPreferred
                        }
                        image {
                            large
                            medium
                        }
                        description
                        gender
                        dateOfBirth {
                            year
                            month
                            day
                        }
                        age
                        bloodType
                        siteUrl
                        favourites
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["characters"].clone();
        let characters: Vec<Character> = serde_json::from_value(data)?;
        Ok(characters)
    }
}
