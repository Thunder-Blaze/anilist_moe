use crate::endpoints::Vth;
use crate::{client::AniListClient, queries::common};
use crate::errors::AniListError;
use serde::Serialize;
use serde_json::json;

#[derive(Default, Serialize)]
pub struct ToggleLikeOptions {
    pub id: i32,
}

#[derive(Default, Serialize)]
pub struct ToggleFollowOptions {
    #[serde(rename = "userId")]
    pub user_id: i32,
}

#[derive(Default, Serialize)]
pub struct ToggleFavouriteOptions {
    #[serde(rename = "animeId", skip_serializing_if = "Option::is_none")]
    pub anime_id: Option<i32>,
    #[serde(rename = "mangaId", skip_serializing_if = "Option::is_none")]
    pub manga_id: Option<i32>,
    #[serde(rename = "characterId", skip_serializing_if = "Option::is_none")]
    pub character_id: Option<i32>,
    #[serde(rename = "staffId", skip_serializing_if = "Option::is_none")]
    pub staff_id: Option<i32>,
    #[serde(rename = "studioId", skip_serializing_if = "Option::is_none")]
    pub studio_id: Option<i32>,
}

pub struct CommonEndpoint(pub(crate) AniListClient);

impl CommonEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self(client)
    }

    pub async fn toggle_like(
        &self,
        options: ToggleLikeOptions,
    ) -> Result<bool, AniListError> {
        let query = common::TOGGLE_LIKE;
        let variables = json!(options);
        let variables_map = self.value_to_hashmap(variables);
        self.0.query_typed(query, Some(&variables_map)).await
    }

    pub async fn toggle_follow(
        &self,
        options: ToggleFollowOptions,
    ) -> Result<bool, AniListError> {
        let query = common::TOGGLE_FOLLOW;
        let variables = json!(options);
        let variables_map = self.value_to_hashmap(variables);
        self.0.query_typed(query, Some(&variables_map)).await
    }

    pub async fn toggle_favourite(
        &self,
        options: ToggleFavouriteOptions,
    ) -> Result<bool, AniListError> {
        let query = common::TOGGLE_FAVOURITE;
        let variables = json!(options);
        let variables_map = self.value_to_hashmap(variables);
        self.0.query_typed(query, Some(&variables_map)).await
    }
}

impl Vth for CommonEndpoint {}
