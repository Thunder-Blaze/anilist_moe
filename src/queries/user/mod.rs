pub const BASIC: &str = include_str!("basic.graphql");

pub const FETCH: &str = include_str!("fetch.graphql");
pub const FETCH_ONE: &str = include_str!("fetch_one.graphql");
/// The authenticated user, with the same fields as [`FETCH_ONE`].
pub const VIEWER: &str = include_str!("viewer.graphql");

pub const FOLLOWERS: &str = include_str!("followers.graphql");
pub const FOLLOWING: &str = include_str!("following.graphql");

pub const FAVORITES: &str = include_str!("favorites.graphql");

pub const MEDIA_LIST: &str = include_str!("media_list.graphql");

pub const STATS: &str = include_str!("stats.graphql");
