use serde::Serialize;
use crate::client::AniListClient;
use crate::errors::AniListError;
use crate::enums::user::UserSort;
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Default, Serialize)]
pub struct UserSearchOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<UserSort>>,
}

pub struct UserEndpoint(pub(crate) AniListClient);

impl UserEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self(client)
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Value, AniListError> {
        let options = UserSearchOptions {
            id: Some(id),
            ..Default::default()
        };
        self.search_users(options).await
    }

    pub async fn get_by_username(&self, name: &str) -> Result<Value, AniListError> {
        let options = UserSearchOptions {
            search: Some(name.to_string()),
            ..Default::default()
        };
        let result = self.search_users(options).await?;
        let users = result
            .get("data")
            .and_then(|d| d.get("Page"))
            .and_then(|p| p.get("users"))
            .and_then(|u| u.as_array());
        if let Some(users) = users {
            for user in users {
                if let Some(username) = user.get("name").and_then(|n| n.as_str()) {
                    if username.eq_ignore_ascii_case(name) {
                        return Ok(json!({"data": {"User": user}}));
                    }
                }
            }
        }
        return Err(AniListError::NotFound);
    }

    pub async fn search(&self, query: &str, page: i32, per_page: i32) -> Result<Value, AniListError> {
        let options = UserSearchOptions {
            search: Some(query.to_string()),
            page: Some(page),
            per_page: Some(per_page),
            ..Default::default()
        };
        self.search_users(options).await
    }

    pub async fn get_most_anime_watched(&self, page: i32, per_page: i32) -> Result<Value, AniListError> {
        let options = UserSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            sort: Some(vec![UserSort::WatchedTimeDesc]),
            ..Default::default()
        };
        self.search_users(options).await
    }

    pub async fn get_most_manga_read(&self, page: i32, per_page: i32) -> Result<Value, AniListError> {
        let options = UserSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            sort: Some(vec![UserSort::ChaptersReadDesc]),
            ..Default::default()
        };
        self.search_users(options).await
    }

    async fn search_users(&self, options: UserSearchOptions) -> Result<Value, AniListError> {
        let query = include_str!("../queries/user/search_users.graphql");
        let variables = json!(options);
        let variables_map = self.value_to_hashmap(variables);
        self.0.query(query, Some(&variables_map)).await
    }

    fn value_to_hashmap(&self, value: Value) -> HashMap<String, Value> {
        match value {
            Value::Object(map) => map.into_iter().collect(),
            _ => HashMap::new(),
        }
    }
}


