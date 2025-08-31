use crate::client::AniListClient;
use crate::error::AniListError;
use crate::models::staff::Staff;
use serde_json::json;
use std::collections::HashMap;

pub struct StaffEndpoint {
    client: AniListClient,
}

impl StaffEndpoint {
    pub(crate) fn new(client: AniListClient) -> Self {
        Self { client }
    }

    pub async fn get_popular(&self, page: i32, per_page: i32) -> Result<Vec<Staff>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    staff(sort: FAVOURITES_DESC) {
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
                        languageV2
                        image {
                            large
                            medium
                        }
                        description
                        primaryOccupations
                        gender
                        dateOfBirth {
                            year
                            month
                            day
                        }
                        dateOfDeath {
                            year
                            month
                            day
                        }
                        age
                        yearsActive
                        homeTown
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
        let data = response["data"]["Page"]["staff"].clone();
        let staff_list: Vec<Staff> = serde_json::from_value(data)?;
        Ok(staff_list)
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Staff, AniListError> {
        let query = r#"
            query ($id: Int) {
                Staff(id: $id) {
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
                    languageV2
                    image {
                        large
                        medium
                    }
                    description
                    primaryOccupations
                    gender
                    dateOfBirth {
                        year
                        month
                        day
                    }
                    dateOfDeath {
                        year
                        month
                        day
                    }
                    age
                    yearsActive
                    homeTown
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
        let data = response["data"]["Staff"].clone();
        let staff: Staff = serde_json::from_value(data)?;
        Ok(staff)
    }

    pub async fn search(
        &self,
        search: &str,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Staff>, AniListError> {
        let query = r#"
            query ($search: String, $page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    staff(search: $search) {
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
                        languageV2
                        image {
                            large
                            medium
                        }
                        description
                        primaryOccupations
                        gender
                        dateOfBirth {
                            year
                            month
                            day
                        }
                        age
                        yearsActive
                        homeTown
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
        let data = response["data"]["Page"]["staff"].clone();
        let staff_list: Vec<Staff> = serde_json::from_value(data)?;
        Ok(staff_list)
    }

    pub async fn get_today_birthday(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Staff>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    staff(sort: FAVOURITES_DESC, isBirthday: true) {
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
                        primaryOccupations
                        gender
                        dateOfBirth {
                            year
                            month
                            day
                        }
                        age
                        yearsActive
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
        let data = response["data"]["Page"]["staff"].clone();
        let staff_list: Vec<Staff> = serde_json::from_value(data)?;

        Ok(staff_list)
    }

    pub async fn get_most_favorited(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Staff>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    staff(sort: FAVOURITES_DESC) {
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
                        primaryOccupations
                        gender
                        dateOfBirth {
                            year
                            month
                            day
                        }
                        age
                        yearsActive
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
        let data = response["data"]["Page"]["staff"].clone();
        let staff_list: Vec<Staff> = serde_json::from_value(data)?;
        Ok(staff_list)
    }
}
