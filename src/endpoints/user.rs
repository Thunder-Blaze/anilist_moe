use crate::client::AniListClient;
use crate::error::AniListError;
use crate::models::media_list::MediaList;
use crate::models::user::User;
use crate::queries;
use serde_json::json;
use std::collections::HashMap;

pub struct UserEndpoint {
    client: AniListClient,
}

impl UserEndpoint {
    pub(crate) fn new(client: AniListClient) -> Self {
        Self { client }
    }

    /// Get the currently authenticated user (requires token)
    pub async fn get_current_user(&self) -> Result<User, AniListError> {
        let query = r#"
            query {
                Viewer {
                    id
                    name
                    about
                    avatar {
                        large
                        medium
                    }
                    bannerImage
                    options {
                        titleLanguage
                        displayAdultContent
                        airingNotifications
                        profileColor
                        timezone
                        activityMergeTime
                        staffNameLanguage
                    }
                    mediaListOptions {
                        scoreFormat
                        rowOrder
                        animeList {
                            sectionOrder
                            splitCompletedSectionByFormat
                            customLists
                            advancedScoring
                            advancedScoringEnabled
                        }
                        mangaList {
                            sectionOrder
                            splitCompletedSectionByFormat
                            customLists
                            advancedScoring
                            advancedScoringEnabled
                        }
                    }
                    favourites {
                        anime {
                            nodes {
                                id
                                title {
                                    userPreferred
                                }
                            }
                        }
                        manga {
                            nodes {
                                id
                                title {
                                    userPreferred
                                }
                            }
                        }
                        characters {
                            nodes {
                                id
                                name {
                                    userPreferred
                                }
                            }
                        }
                        staff {
                            nodes {
                                id
                                name {
                                    userPreferred
                                }
                            }
                        }
                        studios {
                            nodes {
                                id
                                name
                            }
                        }
                    }
                    statistics {
                        anime {
                            count
                            meanScore
                            standardDeviation
                            minutesWatched
                            episodesWatched
                        }
                        manga {
                            count
                            meanScore
                            standardDeviation
                            chaptersRead
                            volumesRead
                        }
                    }
                    unreadNotificationCount
                    siteUrl
                    donatorTier
                    donatorBadge
                    moderatorRoles
                    createdAt
                    updatedAt
                }
            }
        "#;

        let response = self.client.query(query, None).await?;
        let data = response["data"]["Viewer"].clone();
        let user: User = serde_json::from_value(data)?;
        Ok(user)
    }

    /// Get the current user's anime list (requires token)
    pub async fn get_current_user_anime_list(
        &self,
        status: Option<&str>,
    ) -> Result<Vec<MediaList>, AniListError> {
        // For now, we'll use a simpler approach
        let query = r#"
            query ($userId: Int, $type: MediaType, $status: MediaListStatus) {
                MediaListCollection(userId: $userId, type: $type, status: $status) {
                    lists {
                        entries {
                            id
                            userId
                            mediaId
                            status
                            score
                            progress
                            progressVolumes
                            repeat
                            priority
                            private
                            notes
                            hiddenFromStatusLists
                            startedAt {
                                year
                                month
                                day
                            }
                            completedAt {
                                year
                                month
                                day
                            }
                            updatedAt
                            createdAt
                            media {
                                id
                                title {
                                    romaji
                                    english
                                    native
                                    userPreferred
                                }
                                coverImage {
                                    extraLarge
                                    large
                                    medium
                                    color
                                }
                                format
                                status
                                episodes
                                season
                                seasonYear
                                averageScore
                                genres
                            }
                        }
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("type".to_string(), json!("ANIME"));

        if let Some(status) = status {
            variables.insert("status".to_string(), json!(status.to_uppercase()));
        }

        let response = self.client.query(query, Some(variables)).await?;

        // Extract entries from all lists
        let mut all_entries = Vec::new();
        if let Some(lists) = response["data"]["MediaListCollection"]["lists"].as_array() {
            for list in lists {
                if let Some(entries) = list["entries"].as_array() {
                    for entry in entries {
                        if let Ok(media_list) = serde_json::from_value::<MediaList>(entry.clone()) {
                            all_entries.push(media_list);
                        }
                    }
                }
            }
        }

        Ok(all_entries)
    }

    /// Get user by ID
    pub async fn get_by_id(&self, id: i32) -> Result<User, AniListError> {
        let query = r#"
            query ($id: Int) {
                User(id: $id) {
                    id
                    name
                    about
                    avatar {
                        large
                        medium
                    }
                    bannerImage
                    isFollowing
                    isFollower
                    isBlocked
                    options {
                        titleLanguage
                        displayAdultContent
                        airingNotifications
                        profileColor
                        timezone
                        activityMergeTime
                        staffNameLanguage
                    }
                    mediaListOptions {
                        scoreFormat
                        rowOrder
                        animeList {
                            sectionOrder
                            splitCompletedSectionByFormat
                            customLists
                            advancedScoring
                            advancedScoringEnabled
                        }
                        mangaList {
                            sectionOrder
                            splitCompletedSectionByFormat
                            customLists
                            advancedScoring
                            advancedScoringEnabled
                        }
                    }
                    favourites {
                        anime {
                            nodes {
                                id
                                title {
                                    userPreferred
                                }
                            }
                        }
                        manga {
                            nodes {
                                id
                                title {
                                    userPreferred
                                }
                            }
                        }
                        characters {
                            nodes {
                                id
                                name {
                                    userPreferred
                                }
                            }
                        }
                        staff {
                            nodes {
                                id
                                name {
                                    userPreferred
                                }
                            }
                        }
                        studios {
                            nodes {
                                id
                                name
                            }
                        }
                    }
                    statistics {
                        anime {
                            count
                            meanScore
                            standardDeviation
                            minutesWatched
                            episodesWatched
                        }
                        manga {
                            count
                            meanScore
                            standardDeviation
                            chaptersRead
                            volumesRead
                        }
                    }
                    unreadNotificationCount
                    siteUrl
                    donatorTier
                    donatorBadge
                    moderatorRoles
                    createdAt
                    updatedAt
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("id".to_string(), json!(id));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["User"].clone();
        let user: User = serde_json::from_value(data)?;
        Ok(user)
    }

    /// Get user by name
    pub async fn get_by_name(&self, name: &str) -> Result<User, AniListError> {
        let query = r#"
            query ($name: String) {
                User(name: $name) {
                    id
                    name
                    about
                    avatar {
                        large
                        medium
                    }
                    bannerImage
                    isFollowing
                    isFollower
                    isBlocked
                    options {
                        titleLanguage
                        displayAdultContent
                        airingNotifications
                        profileColor
                        timezone
                        activityMergeTime
                        staffNameLanguage
                    }
                    mediaListOptions {
                        scoreFormat
                        rowOrder
                        animeList {
                            sectionOrder
                            splitCompletedSectionByFormat
                            customLists
                            advancedScoring
                            advancedScoringEnabled
                        }
                        mangaList {
                            sectionOrder
                            splitCompletedSectionByFormat
                            customLists
                            advancedScoring
                            advancedScoringEnabled
                        }
                    }
                    favourites {
                        anime {
                            nodes {
                                id
                                title {
                                    userPreferred
                                }
                            }
                        }
                        manga {
                            nodes {
                                id
                                title {
                                    userPreferred
                                }
                            }
                        }
                        characters {
                            nodes {
                                id
                                name {
                                    userPreferred
                                }
                            }
                        }
                        staff {
                            nodes {
                                id
                                name {
                                    userPreferred
                                }
                            }
                        }
                        studios {
                            nodes {
                                id
                                name
                            }
                        }
                    }
                    statistics {
                        anime {
                            count
                            meanScore
                            standardDeviation
                            minutesWatched
                            episodesWatched
                        }
                        manga {
                            count
                            meanScore
                            standardDeviation
                            chaptersRead
                            volumesRead
                        }
                    }
                    unreadNotificationCount
                    siteUrl
                    donatorTier
                    donatorBadge
                    moderatorRoles
                    createdAt
                    updatedAt
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("name".to_string(), json!(name));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["User"].clone();
        let user: User = serde_json::from_value(data)?;
        Ok(user)
    }

    /// Search users by name
    pub async fn search(
        &self,
        search: &str,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<User>, AniListError> {
        let query = r#"
            query ($search: String, $page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    users(search: $search) {
                        id
                        name
                        about
                        avatar {
                            large
                            medium
                        }
                        bannerImage
                        statistics {
                            anime {
                                count
                                meanScore
                                minutesWatched
                                episodesWatched
                            }
                            manga {
                                count
                                meanScore
                                chaptersRead
                                volumesRead
                            }
                        }
                        siteUrl
                        donatorTier
                        createdAt
                        updatedAt
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("search".to_string(), json!(search));
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["users"].clone();
        let users: Vec<User> = serde_json::from_value(data)?;
        Ok(users)
    }

    /// Get users with most anime watched
    pub async fn get_most_anime_watched(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<User>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    users(sort: WATCHED_TIME_DESC) {
                        id
                        name
                        avatar {
                            large
                            medium
                        }
                        statistics {
                            anime {
                                count
                                meanScore
                                minutesWatched
                                episodesWatched
                            }
                            manga {
                                count
                                meanScore
                                chaptersRead
                                volumesRead
                            }
                        }
                        siteUrl
                        donatorTier
                        createdAt
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["users"].clone();
        let users: Vec<User> = serde_json::from_value(data)?;
        Ok(users)
    }

    /// Get users with most manga read
    pub async fn get_most_manga_read(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<User>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    users(sort: CHAPTERS_READ_DESC) {
                        id
                        name
                        avatar {
                            large
                            medium
                        }
                        statistics {
                            anime {
                                count
                                meanScore
                                minutesWatched
                                episodesWatched
                            }
                            manga {
                                count
                                meanScore
                                chaptersRead
                                volumesRead
                            }
                        }
                        siteUrl
                        donatorTier
                        createdAt
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["users"].clone();
        let users: Vec<User> = serde_json::from_value(data)?;
        Ok(users)
    }

    /// Toggle follow/unfollow a user (requires authentication)
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user to follow/unfollow
    ///
    /// # Returns
    /// Returns the updated User object with follow status
    ///
    /// # Errors
    /// * `AniListError::Unauthorized` - If no authentication token is provided
    /// * `AniListError::Network` - If there's a network connectivity issue
    /// * `AniListError::ApiError` - If the AniList API returns an error
    ///
    /// # Example
    /// ```rust
    /// let user = client.user().toggle_follow(123456).await?;
    /// println!("User {} follow status: {}", user.name, user.is_following.unwrap_or(false));
    /// ```
    pub async fn toggle_follow(&self, user_id: i32) -> Result<User, AniListError> {
        let query = queries::user::TOGGLE_FOLLOW;

        let mut variables = HashMap::new();
        variables.insert("userId".to_string(), json!(user_id));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["ToggleFollow"].clone();
        let user: User = serde_json::from_value(data)?;
        Ok(user)
    }

    /// Toggle favorite anime/manga for the authenticated user
    ///
    /// # Arguments
    /// * `anime_id` - The ID of the anime to favorite/unfavorite (optional)
    /// * `manga_id` - The ID of the manga to favorite/unfavorite (optional)
    ///
    /// # Returns
    /// Returns a simple boolean indicating success
    ///
    /// # Errors
    /// * `AniListError::Unauthorized` - If no authentication token is provided
    /// * `AniListError::InvalidInput` - If neither anime_id nor manga_id is provided
    /// * `AniListError::Network` - If there's a network connectivity issue
    /// * `AniListError::ApiError` - If the AniList API returns an error
    ///
    /// # Example
    /// ```rust
    /// // Favorite an anime
    /// let success = client.user().toggle_favorite(Some(21), None).await?;
    ///
    /// // Favorite a manga
    /// let success = client.user().toggle_favorite(None, Some(30013)).await?;
    /// ```
    pub async fn toggle_favorite(
        &self,
        anime_id: Option<i32>,
        manga_id: Option<i32>,
    ) -> Result<bool, AniListError> {
        if anime_id.is_none() && manga_id.is_none() {
            return Err(AniListError::BadRequest {
                message: "Either anime_id or manga_id must be provided".to_string(),
            });
        }

        let query = queries::user::TOGGLE_FAVORITE;

        let mut variables = HashMap::new();
        if let Some(id) = anime_id {
            variables.insert("animeId".to_string(), json!(id));
        }
        if let Some(id) = manga_id {
            variables.insert("mangaId".to_string(), json!(id));
        }

        let response = self.client.query(query, Some(variables)).await?;
        // The mutation returns the updated favourites object, but we'll just return success
        Ok(response["data"]["ToggleFavourite"].is_object())
    }
}
