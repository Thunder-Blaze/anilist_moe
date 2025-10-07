//! # AniList Client
//!
//! The main client interface for interacting with the AniList GraphQL API.
//! This module provides the [`AniListClient`] struct which serves as the entry point
//! for all API operations, handling authentication, rate limiting, and request management.

use crate::endpoints::{
    ActivityEndpoint, AiringEndpoint, AnimeEndpoint, CharacterEndpoint, ForumEndpoint,
    MangaEndpoint, NotificationEndpoint, RecommendationEndpoint, ReviewEndpoint, StaffEndpoint,
    StudioEndpoint, UserEndpoint,
};
use crate::error::AniListError;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

/// The base URL for the AniList GraphQL API endpoint
const ANILIST_API_URL: &str = "https://graphql.anilist.co";

/// The main client for interacting with the AniList API.
///
/// This client provides access to all AniList endpoints through a modular design.
/// It can be used in both authenticated and unauthenticated modes, with automatic
/// rate limiting handling and comprehensive error management.
///
/// # Rate Limiting
///
/// AniList enforces a rate limit of 90 requests per minute. The client automatically
/// handles rate limit responses and provides appropriate error information when limits
/// are exceeded.
///
/// # Authentication
///
/// Some endpoints require authentication. Create an authenticated client using
/// [`AniListClient::with_token`] with an access token from the AniList developer settings.
///
/// # Examples
///
/// ## Unauthenticated Usage
///
/// ```rust
/// use anilist_moe::AniListClient;
///
/// let client = AniListClient::new();
/// // Access public endpoints
/// let trending_anime = client.anime().get_trending(1, 10).await?;
/// ```
///
/// ## Authenticated Usage
///
/// ```rust
/// use anilist_moe::AniListClient;
///
/// let client = AniListClient::with_token("your_token".to_string());
/// // Access both public and private endpoints
/// let user_profile = client.user().get_current_user().await?;
/// ```
#[derive(Clone)]
pub struct AniListClient {
    /// The HTTP client used for making requests
    client: Client,
    /// Optional authentication token for authenticated requests
    token: Option<String>,
}

impl AniListClient {
    /// Creates a new unauthenticated AniList client.
    ///
    /// This client can access all public endpoints but cannot perform operations
    /// that require authentication such as posting activities, managing lists,
    /// or accessing private user data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use anilist_moe::AniListClient;
    ///
    /// let client = AniListClient::new();
    ///
    /// // Can access public endpoints
    /// let popular_anime = client.anime().get_popular(1, 10).await?;
    /// let trending_manga = client.manga().get_trending(1, 5).await?;
    /// ```
    ///
    /// # See Also
    ///
    /// - [`AniListClient::with_token`] for authenticated access
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            token: None,
        }
    }

    /// Creates a new authenticated AniList client with the provided access token.
    ///
    /// This client can access both public and private endpoints, allowing for
    /// full interaction with the AniList API including posting content, managing
    /// user lists, and accessing private data.
    ///
    /// # Parameters
    ///
    /// * `token` - A valid AniList access token obtained from the
    ///   [AniList Developer Settings](https://anilist.co/settings/developer)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use anilist_moe::AniListClient;
    /// use std::env;
    ///
    /// let token = env::var("ANILIST_TOKEN").expect("ANILIST_TOKEN not set");
    /// let client = AniListClient::with_token(token);
    ///
    /// // Can access both public and private endpoints
    /// let user_profile = client.user().get_current_user().await?;
    /// let notifications = client.notification().get_notifications(1, 10).await?;
    /// ```
    ///
    /// # Authentication Requirements
    ///
    /// The following operations require authentication:
    /// - User profile management and statistics
    /// - Media list operations (add, update, delete entries)
    /// - Social features (activities, following, favorites)
    /// - Forum operations (posting, commenting, moderation)
    /// - Review and recommendation management
    /// - Notification management
    ///
    /// # See Also
    ///
    /// - [`AniListClient::new`] for unauthenticated access
    pub fn with_token(token: String) -> Self {
        Self {
            client: Client::new(),
            token: Some(token),
        }
    }

    /// Gets an interface to the anime-related endpoints.
    ///
    /// Provides access to anime search, trending data, popular series, seasonal content,
    /// detailed anime information, and airing schedules.
    ///
    /// # Available Operations
    ///
    /// - Search anime by title, genre, or other criteria
    /// - Get trending and popular anime
    /// - Retrieve detailed anime information by ID
    /// - Browse anime by season and year
    /// - Get currently airing anime
    /// - Access top-rated anime
    ///
    /// # Examples
    ///
    /// ```rust
    /// let client = AniListClient::new();
    ///
    /// // Search for anime
    /// let results = client.anime().search("Attack on Titan", 1, 5).await?;
    ///
    /// // Get trending anime
    /// let trending = client.anime().get_trending(1, 10).await?;
    ///
    /// // Get anime by ID
    /// let anime = client.anime().get_by_id(16498).await?;
    /// ```
    ///
    /// # Authentication
    ///
    /// Most anime endpoints are publicly accessible and do not require authentication.
    ///
    /// # See Also
    ///
    /// - [`crate::endpoints::anime`] for detailed endpoint documentation
    pub fn anime(&self) -> AnimeEndpoint {
        AnimeEndpoint::new(self.clone())
    }

    /// Gets an interface to the manga-related endpoints.
    ///
    /// Provides access to manga search, trending data, popular series, detailed
    /// manga information, and publication status tracking.
    ///
    /// # Available Operations
    ///
    /// - Search manga by title, author, or other criteria
    /// - Get trending and popular manga
    /// - Retrieve detailed manga information by ID
    /// - Get currently releasing manga
    /// - Access top-rated manga
    /// - Browse completed manga series
    ///
    /// # Examples
    ///
    /// ```rust
    /// let client = AniListClient::new();
    ///
    /// // Search for manga
    /// let results = client.manga().search("One Piece", 1, 5).await?;
    ///
    /// // Get popular manga
    /// let popular = client.manga().get_popular(1, 10).await?;
    /// ```
    ///
    /// # Authentication
    ///
    /// Most manga endpoints are publicly accessible and do not require authentication.
    ///
    /// # See Also
    ///
    /// - [`crate::endpoints::manga`] for detailed endpoint documentation
    pub fn manga(&self) -> MangaEndpoint {
        MangaEndpoint::new(self.clone())
    }

    /// Gets an interface to the character-related endpoints.
    ///
    /// Provides access to character search, popular characters, detailed character
    /// information, voice actor data, and media appearance tracking.
    ///
    /// # Available Operations
    ///
    /// - Search characters by name
    /// - Get popular characters
    /// - Retrieve detailed character information by ID
    /// - Access character media appearances
    /// - Get voice actor information
    ///
    /// # Examples
    ///
    /// ```rust
    /// let client = AniListClient::new();
    ///
    /// // Search for characters
    /// let results = client.character().search("Eren", 1, 5).await?;
    ///
    /// // Get character by ID
    /// let character = client.character().get_by_id(40882).await?;
    /// ```
    ///
    /// # Authentication
    ///
    /// Character endpoints are publicly accessible and do not require authentication.
    ///
    /// # See Also
    ///
    /// - [`crate::endpoints::character`] for detailed endpoint documentation
    pub fn character(&self) -> CharacterEndpoint {
        CharacterEndpoint::new(self.clone())
    }

    /// Gets an interface to the staff-related endpoints.
    ///
    /// Provides access to staff search, popular staff members, detailed staff
    /// information, and production history tracking.
    ///
    /// # Available Operations
    ///
    /// - Search staff by name
    /// - Get popular staff members
    /// - Retrieve detailed staff information by ID
    /// - Access staff production history
    /// - Get role and position information
    ///
    /// # Examples
    ///
    /// ```rust
    /// let client = AniListClient::new();
    ///
    /// // Search for staff
    /// let results = client.staff().search("Yuki Kaji", 1, 5).await?;
    ///
    /// // Get staff by ID
    /// let staff = client.staff().get_by_id(95269).await?;
    /// ```
    ///
    /// # Authentication
    ///
    /// Staff endpoints are publicly accessible and do not require authentication.
    ///
    /// # See Also
    ///
    /// - [`crate::endpoints::staff`] for detailed endpoint documentation
    pub fn staff(&self) -> StaffEndpoint {
        StaffEndpoint::new(self.clone())
    }

    /// Gets an interface to the user-related endpoints.
    ///
    /// Provides access to user profiles, statistics, media lists, favorites,
    /// following relationships, and social interactions.
    ///
    /// # Available Operations
    ///
    /// - Get user profiles and statistics
    /// - Access and manage media lists
    /// - Manage favorites and following
    /// - View user activity and social data
    /// - Update user settings (authenticated)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let client = AniListClient::with_token(token);
    ///
    /// // Get current user (requires authentication)
    /// let user = client.user().get_current_user().await?;
    ///
    /// // Get user by ID (public)
    /// let user = client.user().get_by_id(123456).await?;
    ///
    /// // Search users (public)
    /// let users = client.user().search("username", 1, 5).await?;
    /// ```
    ///
    /// # Authentication
    ///
    /// Some user operations require authentication:
    /// - Getting current user profile and private data
    /// - Managing media lists and favorites
    /// - Following/unfollowing users
    /// - Updating user settings
    ///
    /// # See Also
    ///
    /// - [`crate::endpoints::user`] for detailed endpoint documentation
    pub fn user(&self) -> UserEndpoint {
        UserEndpoint::new(self.clone())
    }

    /// Gets an interface to the studio-related endpoints.
    ///
    /// Provides access to animation studio information, production history,
    /// and studio search functionality.
    ///
    /// # Available Operations
    ///
    /// - Search studios by name
    /// - Get detailed studio information by ID
    /// - Access studio production history
    /// - Browse anime produced by specific studios
    ///
    /// # Examples
    ///
    /// ```rust
    /// let client = AniListClient::new();
    ///
    /// // Search for studios
    /// let studios = client.studio().search("Studio Ghibli", 1, 5).await?;
    ///
    /// // Get studio by ID
    /// let studio = client.studio().get_by_id(21).await?;
    /// ```
    ///
    /// # Authentication
    ///
    /// Studio endpoints are publicly accessible and do not require authentication.
    ///
    /// # See Also
    ///
    /// - [`crate::endpoints::studio`] for detailed endpoint documentation
    pub fn studio(&self) -> StudioEndpoint {
        StudioEndpoint::new(self.clone())
    }

    /// Gets an interface to the forum-related endpoints.
    ///
    /// Provides access to AniList's forum system including threads, comments,
    /// categories, and moderation features.
    ///
    /// # Available Operations
    ///
    /// - Browse forum threads and categories
    /// - Create and manage forum threads (authenticated)
    /// - Post and manage comments (authenticated)
    /// - Search forum content
    /// - Access moderation features (authenticated with permissions)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let client = AniListClient::with_token(token);
    ///
    /// // Get recent threads (public)
    /// let threads = client.forum().get_recent_threads(1, 10).await?;
    ///
    /// // Create a thread (requires authentication)
    /// let thread = client.forum().create_thread("Title", "Content", 1).await?;
    /// ```
    ///
    /// # Authentication
    ///
    /// Forum operations requiring authentication:
    /// - Creating threads and comments
    /// - Editing and deleting own content
    /// - Voting and reactions
    /// - Moderation actions (with appropriate permissions)
    ///
    /// # See Also
    ///
    /// - [`crate::endpoints::forum`] for detailed endpoint documentation
    pub fn forum(&self) -> ForumEndpoint {
        ForumEndpoint::new(self.clone())
    }

    /// Gets an interface to the activity-related endpoints.
    ///
    /// Provides access to AniList's social activity system including text posts,
    /// list updates, replies, likes, and activity feeds.
    ///
    /// # Available Operations
    ///
    /// - View public activity feeds
    /// - Post text activities (authenticated)
    /// - Reply to activities (authenticated)
    /// - Like and unlike activities (authenticated)
    /// - Get user-specific activity feeds
    /// - Manage activity subscriptions (authenticated)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let client = AniListClient::with_token(token);
    ///
    /// // Get global activity feed (public)
    /// let activities = client.activity().get_global_feed(1, 10).await?;
    ///
    /// // Post a text activity (requires authentication)
    /// let activity = client.activity().post_text_activity("Hello world!".to_string()).await?;
    ///
    /// // Like an activity (requires authentication)
    /// client.activity().toggle_like(activity.id).await?;
    /// ```
    ///
    /// # Authentication
    ///
    /// Activity operations requiring authentication:
    /// - Posting text activities and status updates
    /// - Replying to activities
    /// - Liking and unliking activities
    /// - Managing activity subscriptions and notifications
    /// - Accessing private user activity feeds
    ///
    /// # See Also
    ///
    /// - [`crate::endpoints::activity`] for detailed endpoint documentation
    pub fn activity(&self) -> ActivityEndpoint {
        ActivityEndpoint::new(self.clone())
    }

    /// Gets an interface to the review-related endpoints.
    ///
    /// Provides access to user reviews of anime and manga, including creation,
    /// management, rating, and discovery of reviews.
    ///
    /// # Available Operations
    ///
    /// - Browse and search reviews
    /// - Get detailed review information
    /// - Create and edit reviews (authenticated)
    /// - Rate and vote on reviews (authenticated)
    /// - Delete own reviews (authenticated)
    /// - Get user-specific review history
    ///
    /// # Examples
    ///
    /// ```rust
    /// let client = AniListClient::with_token(token);
    ///
    /// // Get reviews for an anime (public)
    /// let reviews = client.review().get_reviews_for_media(16498, 1, 5).await?;
    ///
    /// // Create a review (requires authentication)
    /// let review = client.review().create_review(
    ///     16498,
    ///     "Great anime!".to_string(),
    ///     "Detailed review text...".to_string(),
    ///     85,
    ///     false
    /// ).await?;
    /// ```
    ///
    /// # Authentication
    ///
    /// Review operations requiring authentication:
    /// - Creating, editing, and deleting reviews
    /// - Rating and voting on reviews
    /// - Managing review privacy settings
    /// - Accessing draft reviews
    ///
    /// # See Also
    ///
    /// - [`crate::endpoints::review`] for detailed endpoint documentation
    pub fn review(&self) -> ReviewEndpoint {
        ReviewEndpoint::new(self.clone())
    }

    /// Gets an interface to the recommendation-related endpoints.
    ///
    /// Provides access to anime and manga recommendations, allowing users to
    /// discover similar content and manage their own recommendations.
    ///
    /// # Available Operations
    ///
    /// - Browse and search recommendations
    /// - Get recommendations for specific media
    /// - Create and manage recommendations (authenticated)
    /// - Rate recommendation usefulness (authenticated)
    /// - Get user-specific recommendation history
    ///
    /// # Examples
    ///
    /// ```rust
    /// let client = AniListClient::with_token(token);
    ///
    /// // Get recommendations for an anime (public)
    /// let recommendations = client.recommendation().get_recommendations_for_media(16498, 1, 5).await?;
    ///
    /// // Create a recommendation (requires authentication)
    /// let recommendation = client.recommendation().create_recommendation(
    ///     16498,  // source media ID
    ///     15125,  // recommended media ID
    ///     "Both have great action and character development".to_string()
    /// ).await?;
    /// ```
    ///
    /// # Authentication
    ///
    /// Recommendation operations requiring authentication:
    /// - Creating and editing recommendations
    /// - Rating recommendation usefulness
    /// - Deleting own recommendations
    /// - Managing recommendation privacy
    ///
    /// # See Also
    ///
    /// - [`crate::endpoints::recommendation`] for detailed endpoint documentation
    pub fn recommendation(&self) -> RecommendationEndpoint {
        RecommendationEndpoint::new(self.clone())
    }

    /// Gets an interface to the airing schedule endpoints.
    ///
    /// Provides access to anime airing schedules, upcoming episodes, recently
    /// aired content, and time-based filtering of airing data.
    ///
    /// # Available Operations
    ///
    /// - Get upcoming episode schedules
    /// - View recently aired episodes
    /// - Filter episodes by date ranges
    /// - Get airing schedules for specific anime
    /// - Track next episode information
    /// - Browse today's airing episodes
    ///
    /// # Examples
    ///
    /// ```rust
    /// let client = AniListClient::new();
    ///
    /// // Get upcoming episodes
    /// let upcoming = client.airing().get_upcoming_episodes(1, 10).await?;
    ///
    /// // Get today's episodes
    /// let today = client.airing().get_today_episodes(1, 10).await?;
    ///
    /// // Get next episode for specific anime
    /// let next_episode = client.airing().get_next_episode(16498).await?;
    /// ```
    ///
    /// # Authentication
    ///
    /// Airing schedule endpoints are publicly accessible and do not require authentication.
    ///
    /// # See Also
    ///
    /// - [`crate::endpoints::airing`] for detailed endpoint documentation
    pub fn airing(&self) -> AiringEndpoint {
        AiringEndpoint::new(self.clone())
    }

    /// Gets an interface to the notification-related endpoints.
    ///
    /// Provides access to user notifications including activity updates,
    /// mentions, follows, and system notifications.
    ///
    /// # Available Operations
    ///
    /// - Get user notifications with filtering
    /// - Mark notifications as read
    /// - Get unread notification counts
    /// - Filter notifications by type
    /// - Manage notification settings
    ///
    /// # Examples
    ///
    /// ```rust
    /// let client = AniListClient::with_token(token);
    ///
    /// // Get recent notifications
    /// let notifications = client.notification().get_notifications(1, 10).await?;
    ///
    /// // Get unread count
    /// let unread_count = client.notification().get_unread_count().await?;
    ///
    /// // Mark notifications as read
    /// client.notification().mark_notifications_as_read(vec![123, 456]).await?;
    /// ```
    ///
    /// # Authentication
    ///
    /// All notification endpoints require authentication as they deal with
    /// user-specific private data.
    ///
    /// # See Also
    ///
    /// - [`crate::endpoints::notification`] for detailed endpoint documentation
    pub fn notification(&self) -> NotificationEndpoint {
        NotificationEndpoint::new(self.clone())
    }

    /// Sets or updates the authentication token for this client.
    ///
    /// This method allows you to add authentication to an existing client instance
    /// or update the token if it has changed (e.g., after token refresh).
    ///
    /// # Parameters
    ///
    /// * `token` - The new AniList access token to use for authenticated requests
    ///
    /// # Examples
    ///
    /// ```rust
    /// use anilist_moe::AniListClient;
    ///
    /// // Start with unauthenticated client
    /// let mut client = AniListClient::new();
    ///
    /// // Later, add authentication
    /// client.set_token("your_access_token".to_string());
    ///
    /// // Now can access authenticated endpoints
    /// let user = client.user().get_current_user().await?;
    ///
    /// // Can also update token if it changes
    /// client.set_token("new_token".to_string());
    /// ```
    ///
    /// # Use Cases
    ///
    /// - **Dynamic Authentication**: Add authentication after client creation
    /// - **Token Refresh**: Update expired tokens during application lifetime
    /// - **User Switching**: Change tokens when switching between user accounts
    /// - **Conditional Authentication**: Enable authentication based on runtime conditions
    ///
    /// # Note
    ///
    /// This method updates the token for the current client instance. If you need
    /// to preserve both authenticated and unauthenticated clients, create separate
    /// client instances instead.
    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    /// Removes authentication from this client.
    ///
    /// After calling this method, the client will no longer include authentication
    /// headers in requests and will only be able to access public endpoints.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use anilist_moe::AniListClient;
    ///
    /// let mut client = AniListClient::with_token("token".to_string());
    ///
    /// // Can access authenticated endpoints
    /// let user = client.user().get_current_user().await?;
    ///
    /// // Remove authentication
    /// client.clear_token();
    ///
    /// // Now only public endpoints work
    /// let anime = client.anime().get_popular(1, 10).await?; // This works
    /// // client.user().get_current_user().await?; // This would fail
    /// ```
    ///
    /// # Use Cases
    ///
    /// - **Logout Functionality**: Remove authentication when user logs out
    /// - **Token Expiry**: Clear invalid tokens to prevent failed requests
    /// - **Privacy Mode**: Temporarily disable authentication for privacy
    /// - **Error Recovery**: Clear potentially corrupted tokens
    pub fn clear_token(&mut self) {
        self.token = None;
    }

    /// Checks if the client currently has an authentication token.
    ///
    /// This method returns `true` if a token is set, but does not validate
    /// whether the token is still valid or has the necessary permissions.
    ///
    /// # Returns
    ///
    /// Returns `true` if an authentication token is set, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use anilist_moe::AniListClient;
    ///
    /// let client = AniListClient::new();
    /// assert!(!client.has_token()); // No token set
    ///
    /// let auth_client = AniListClient::with_token("token".to_string());
    /// assert!(auth_client.has_token()); // Token is set
    ///
    /// let mut mutable_client = AniListClient::new();
    /// mutable_client.set_token("token".to_string());
    /// assert!(mutable_client.has_token()); // Token was added
    ///
    /// mutable_client.clear_token();
    /// assert!(!mutable_client.has_token()); // Token was removed
    /// ```
    ///
    /// # Use Cases
    ///
    /// - **Conditional Logic**: Check authentication before calling protected endpoints
    /// - **UI Updates**: Show/hide authentication-dependent features
    /// - **Error Prevention**: Avoid calls that will fail due to missing authentication
    /// - **State Management**: Track authentication state in applications
    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    /// Executes a GraphQL query against the AniList API.
    ///
    /// This is the low-level method used internally by all endpoint methods to
    /// communicate with the AniList GraphQL API. It handles authentication,
    /// rate limiting, error parsing, and response processing.
    ///
    /// # Parameters
    ///
    /// * `query` - The GraphQL query string to execute
    /// * `variables` - Optional variables to pass with the query
    ///
    /// # Returns
    ///
    /// Returns the raw JSON response from the API on success, or an appropriate
    /// error type on failure.
    ///
    /// # Errors
    ///
    /// This method can return various error types:
    /// - [`AniListError::RateLimit`] when rate limits are exceeded
    /// - [`AniListError::AuthenticationRequired`] for 401 responses
    /// - [`AniListError::AccessDenied`] for 403 responses
    /// - [`AniListError::NotFound`] for 404 responses
    /// - [`AniListError::GraphQL`] for API-level GraphQL errors
    /// - [`AniListError::Network`] for network-related issues
    ///
    /// # Rate Limiting
    ///
    /// This method automatically handles rate limiting by parsing rate limit
    /// headers and returning detailed rate limit information in errors.
    ///
    /// # Authentication
    ///
    /// If the client was created with a token using [`AniListClient::with_token`],
    /// the authentication header will be automatically included in the request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use serde_json::{json, Value};
    ///
    /// let client = AniListClient::new();
    /// let mut variables = HashMap::new();
    /// variables.insert("id".to_string(), json!(16498));
    ///
    /// let query = r#"
    ///     query ($id: Int) {
    ///         Media(id: $id) {
    ///             id
    ///             title { romaji }
    ///         }
    ///     }
    /// "#;
    ///
    /// let response = client.query(query, Some(variables)).await?;
    /// let media = &response["data"]["Media"];
    /// ```
    ///
    /// # Note
    ///
    /// While this method is public, it's primarily intended for internal use.
    /// Most users should use the higher-level endpoint methods instead.
    pub(crate) async fn query(
        &self,
        query: &str,
        variables: Option<HashMap<String, Value>>,
    ) -> Result<Value, AniListError> {
        let mut body = HashMap::new();
        body.insert("query", Value::String(query.to_string()));

        if let Some(vars) = variables {
            body.insert("variables", Value::Object(vars.into_iter().collect()));
        }

        let mut request = self
            .client
            .post(ANILIST_API_URL)
            .header("Content-Type", "application/json");

        // Add authorization header if token is present
        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.json(&body).send().await?;

        // Handle HTTP status codes
        let status = response.status();
        match status.as_u16() {
            200..=299 => {
                // Success, continue processing
            }
            400 => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Bad Request".to_string());
                return Err(AniListError::BadRequest {
                    message: error_text,
                });
            }
            401 => {
                return Err(AniListError::AuthenticationRequired);
            }
            403 => {
                return Err(AniListError::AccessDenied);
            }
            404 => {
                return Err(AniListError::NotFound);
            }
            429 => {
                // Rate limit exceeded - extract rate limit headers
                let headers = response.headers();

                // Try to get detailed rate limit information
                if let (
                    Some(limit_header),
                    Some(remaining_header),
                    Some(reset_header),
                    Some(retry_after_header),
                ) = (
                    headers.get("X-RateLimit-Limit"),
                    headers.get("X-RateLimit-Remaining"),
                    headers.get("X-RateLimit-Reset"),
                    headers.get("Retry-After"),
                ) {
                    let limit = limit_header.to_str().unwrap_or("90").parse().unwrap_or(90);
                    let remaining = remaining_header
                        .to_str()
                        .unwrap_or("0")
                        .parse()
                        .unwrap_or(0);
                    let reset_at = reset_header.to_str().unwrap_or("0").parse().unwrap_or(0);
                    let retry_after = retry_after_header
                        .to_str()
                        .unwrap_or("60")
                        .parse()
                        .unwrap_or(60);

                    return Err(AniListError::RateLimit {
                        limit,
                        remaining,
                        reset_at,
                        retry_after,
                    });
                } else {
                    // Fallback if headers are not available
                    return Err(AniListError::RateLimitSimple);
                }
            }
            500..=599 => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Server Error".to_string());
                return Err(AniListError::ServerError {
                    status: status.as_u16(),
                    message: error_text,
                });
            }
            _ => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown Error".to_string());
                return Err(AniListError::ServerError {
                    status: status.as_u16(),
                    message: error_text,
                });
            }
        }

        let json: Value = response.json().await?;

        // Check for GraphQL errors
        if let Some(errors) = json.get("errors") {
            let error_message = if errors.is_array() {
                errors
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|e| {
                        e.get("message")
                            .and_then(|m| m.as_str())
                            .unwrap_or("Unknown error")
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            } else {
                errors.to_string()
            };

            // Check if it's a rate limit error in GraphQL response
            if error_message.to_lowercase().contains("rate limit")
                || error_message.to_lowercase().contains("too many requests")
            {
                return Err(AniListError::BurstLimit);
            }

            return Err(AniListError::GraphQL {
                message: error_message,
            });
        }

        Ok(json)
    }
}

impl Default for AniListClient {
    fn default() -> Self {
        Self::new()
    }
}
