use serde::Serialize;
use crate::client::AniListClient;
use crate::errors::AniListError;
use crate::enums::staff::StaffSort;
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Default, Serialize)]
pub struct StaffSearchOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<StaffSort>>,
    #[serde(rename = "isBirthday", skip_serializing_if = "Option::is_none")]
    pub is_birthday: Option<bool>,
}

pub struct StaffEndpoint(pub(crate) AniListClient);

impl StaffEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self(client)
    }

    pub async fn search_staff(
        &self,
        options: StaffSearchOptions,
    ) -> Result<Value, AniListError> {
        let query = include_str!("../queries/staff/search_staff.graphql");
        let variables = json!(options);
        let variables_map = self.value_to_hashmap(variables);
        self.0.query(query, Some(&variables_map)).await
    }

    pub async fn get_staff_by_id(&self, id: i32) -> Result<Value, AniListError> {
        let options = StaffSearchOptions {
            id: Some(id),
            ..Default::default()
        };
        self.search_staff(options).await
    }

    fn value_to_hashmap(&self, value: Value) -> HashMap<String, Value> {
        match value {
            Value::Object(map) => map.into_iter().collect(),
            _ => HashMap::new(),
        }
    }
}


