use crate::enums::likable::LikeableType;
use crate::errors::AniListError;
use crate::objects::responses::{
    ToggleFavouriteResponse, ToggleFollowResponse, ToggleLikeResponse,
};
use crate::{client::AniListClient, queries::common};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub struct ToggleLikeOptions {
    pub id: i32,
    #[serde(rename = "type")]
    pub like_type: LikeableType,
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

pub struct CommonEndpoint {
    pub client: AniListClient,
}

impl CommonEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self { client }
    }

    pub async fn toggle_like(
        &self,
        options: ToggleLikeOptions,
    ) -> Result<ToggleLikeResponse, AniListError> {
        let query = common::TOGGLE_LIKE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn toggle_follow(
        &self,
        options: ToggleFollowOptions,
    ) -> Result<ToggleFollowResponse, AniListError> {
        let query = common::TOGGLE_FOLLOW;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn toggle_favourite(
        &self,
        options: ToggleFavouriteOptions,
    ) -> Result<ToggleFavouriteResponse, AniListError> {
        let query = common::TOGGLE_FAVOURITE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }
}
