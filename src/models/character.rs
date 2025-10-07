//! # Character Data Models
//!
//! This module contains data structures representing character information
//! as returned by the AniList API, including character details, names, and images.

use super::FuzzyDate;
use serde::{Deserialize, Serialize};

/// Represents a character entry from AniList.
///
/// This struct contains comprehensive information about an anime or manga character,
/// including personal details, images, and statistics. Characters can be from any
/// media type and may have extensive biographical information.
///
/// # Field Descriptions
///
/// ## Identification
/// - `id`: Unique AniList identifier for this character
/// - `name`: Multi-part name information (first, middle, last, native, alternative)
/// - `site_url`: Direct link to this character's AniList page
///
/// ## Visual Elements
/// - `image`: Character portrait images in multiple sizes
///
/// ## Personal Information
/// - `description`: Character biography and background (may contain HTML)
/// - `gender`: Character's gender if specified
/// - `date_of_birth`: Character's birthday (if known)
/// - `age`: Character's age (can be a range or specific age)
/// - `blood_type`: Character's blood type (common in anime/manga)
///
/// ## User Interaction
/// - `is_favourite`: Whether the authenticated user has favorited this character
/// - `is_favourite_blocked`: Whether favoriting is blocked for this character
/// - `favourites`: Total number of users who have favorited this character
///
/// ## Moderation
/// - `mod_notes`: Moderator notes (visible to moderators only)
///
/// # Examples
///
/// ```rust
/// use anilist_moe::AniListClient;
///
/// let client = AniListClient::new();
/// let character = client.character().get_by_id(40882).await?;
///
/// if let Some(name) = &character.name {
///     println!("Character: {} {}",
///         name.first.as_deref().unwrap_or(""),
///         name.last.as_deref().unwrap_or("")
///     );
/// }
///
/// println!("Favorites: {}", character.favourites.unwrap_or(0));
///
/// if let Some(description) = &character.description {
///     println!("Description: {}", description);
/// }
/// ```
///
/// # Note
///
/// Many fields may be `None` depending on the completeness of character data
/// and whether the information is available or disclosed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    /// Unique identifier for this character on AniList
    pub id: i32,

    /// Multi-part name information for the character
    pub name: Option<CharacterName>,

    /// Character portrait images in various sizes
    pub image: Option<CharacterImage>,

    /// Character biography and background information (may contain HTML)
    pub description: Option<String>,

    /// Character's gender (if specified)
    pub gender: Option<String>,

    /// Character's date of birth (if known)
    #[serde(rename = "dateOfBirth")]
    pub date_of_birth: Option<FuzzyDate>,

    /// Character's age (can be specific age or range like "16-17")
    pub age: Option<String>,

    /// Character's blood type (common character detail in anime/manga)
    #[serde(rename = "bloodType")]
    pub blood_type: Option<String>,

    /// Whether the authenticated user has favorited this character
    #[serde(rename = "isFavourite")]
    pub is_favourite: Option<bool>,

    /// Whether favoriting is blocked for this character
    #[serde(rename = "isFavouriteBlocked")]
    pub is_favourite_blocked: Option<bool>,

    /// Direct URL to this character's page on AniList
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,

    /// Total number of users who have favorited this character
    pub favourites: Option<i32>,

    /// Moderator notes (only visible to moderators)
    #[serde(rename = "modNotes")]
    pub mod_notes: Option<String>,
}

/// Represents the name information for a character.
///
/// Characters can have complex naming conventions including multiple parts
/// in different languages and scripts, alternative names, and native representations.
///
/// # Examples
///
/// ```rust
/// // Western name format
/// let name = CharacterName {
///     first: Some("Edward".to_string()),
///     middle: None,
///     last: Some("Elric".to_string()),
///     full: Some("Edward Elric".to_string()),
///     native: Some("エドワード・エルリック".to_string()),
///     alternative: Some(vec!["Ed".to_string(), "Fullmetal Alchemist".to_string()]),
///     alternative_spoiler: None,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterName {
    /// First name of the character
    pub first: Option<String>,

    /// Middle name of the character (if applicable)
    pub middle: Option<String>,

    /// Last name/surname of the character
    pub last: Option<String>,

    /// Full name as a single string
    pub full: Option<String>,

    /// Native script representation of the name (e.g., Japanese, Korean, Chinese)
    pub native: Option<String>,

    /// List of alternative names, nicknames, and aliases
    pub alternative: Option<Vec<String>>,

    /// Alternative names that may contain spoilers
    pub alternative_spoiler: Option<Vec<String>>,

    /// User's preferred name format (based on user settings)
    #[serde(rename = "userPreferred")]
    pub user_preferred: Option<String>,
}

/// Represents character image URLs in different sizes.
///
/// Provides character portrait images optimized for different display contexts.
/// Images are hosted on AniList's CDN and are available in multiple resolutions.
///
/// # Examples
///
/// ```rust
/// if let Some(image) = &character.image {
///     // Use large image for detailed view
///     if let Some(large_url) = &image.large {
///         println!("High-res image: {}", large_url);
///     }
///     
///     // Use medium image for list views
///     if let Some(medium_url) = &image.medium {
///         println!("Medium image: {}", medium_url);
///     }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterImage {
    /// Large character image URL (typically 230x350px or larger)
    pub large: Option<String>,

    /// Medium character image URL (typically 92x140px)
    pub medium: Option<String>,
}
