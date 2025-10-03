use crate::enums::likable::LikeableType;
use crate::errors::AniListError;
use crate::objects::responses::{
    ToggleFavouriteResponse, ToggleFollowResponse, ToggleLikeResponse,
};
use crate::{client::AniListClient, queries::common};
use serde::Serialize;
use serde_json::json;

/// Options for toggling a like on various entities.
#[derive(Serialize)]
pub struct ToggleLikeOptions {
    pub id: i32,
    #[serde(rename = "type")]
    pub like_type: LikeableType,
}

/// Options for toggling follow status of a user.
#[derive(Default, Serialize)]
pub struct ToggleFollowOptions {
    #[serde(rename = "userId")]
    pub user_id: i32,
}

/// Options for adding or removing favourites.
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

/// Endpoint for common and general-purpose operations.
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

    // Convenience functions

    /// Like or unlike an activity
    pub async fn like_activity(&self, id: i32) -> Result<ToggleLikeResponse, AniListError> {
        self.toggle_like(ToggleLikeOptions {
            id,
            like_type: LikeableType::Activity,
        })
        .await
    }

    /// Like or unlike an activity reply
    pub async fn like_activity_reply(
        &self,
        id: i32,
    ) -> Result<ToggleLikeResponse, AniListError> {
        self.toggle_like(ToggleLikeOptions {
            id,
            like_type: LikeableType::ActivityReply,
        })
        .await
    }

    /// Like or unlike a thread
    pub async fn like_thread(&self, id: i32) -> Result<ToggleLikeResponse, AniListError> {
        self.toggle_like(ToggleLikeOptions {
            id,
            like_type: LikeableType::Thread,
        })
        .await
    }

    /// Like or unlike a thread comment
    pub async fn like_thread_comment(
        &self,
        id: i32,
    ) -> Result<ToggleLikeResponse, AniListError> {
        self.toggle_like(ToggleLikeOptions {
            id,
            like_type: LikeableType::ThreadComment,
        })
        .await
    }

    /// Follow or unfollow a user
    pub async fn follow_user(&self, user_id: i32) -> Result<ToggleFollowResponse, AniListError> {
        self.toggle_follow(ToggleFollowOptions { user_id }).await
    }

    /// Add or remove anime from favorites
    pub async fn favourite_anime(
        &self,
        anime_id: i32,
    ) -> Result<ToggleFavouriteResponse, AniListError> {
        self.toggle_favourite(ToggleFavouriteOptions {
            anime_id: Some(anime_id),
            ..Default::default()
        })
        .await
    }

    /// Add or remove manga from favorites
    pub async fn favourite_manga(
        &self,
        manga_id: i32,
    ) -> Result<ToggleFavouriteResponse, AniListError> {
        self.toggle_favourite(ToggleFavouriteOptions {
            manga_id: Some(manga_id),
            ..Default::default()
        })
        .await
    }

    /// Add or remove character from favorites
    pub async fn favourite_character(
        &self,
        character_id: i32,
    ) -> Result<ToggleFavouriteResponse, AniListError> {
        self.toggle_favourite(ToggleFavouriteOptions {
            character_id: Some(character_id),
            ..Default::default()
        })
        .await
    }

    /// Add or remove staff from favorites
    pub async fn favourite_staff(
        &self,
        staff_id: i32,
    ) -> Result<ToggleFavouriteResponse, AniListError> {
        self.toggle_favourite(ToggleFavouriteOptions {
            staff_id: Some(staff_id),
            ..Default::default()
        })
        .await
    }

    /// Add or remove studio from favorites
    pub async fn favourite_studio(
        &self,
        studio_id: i32,
    ) -> Result<ToggleFavouriteResponse, AniListError> {
        self.toggle_favourite(ToggleFavouriteOptions {
            studio_id: Some(studio_id),
            ..Default::default()
        })
        .await
    }
}
