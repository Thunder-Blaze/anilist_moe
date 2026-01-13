use crate::objects::common::PageInfo;
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::fmt;
use std::marker::PhantomData;

/// Top-level GraphQL response wrapper.
///
/// This struct wraps all responses from the AniList GraphQL API.
/// The generic type `T` represents the actual data returned by the query.
///
/// # Type Parameters
///
/// * `T` - The type of data contained in the response
///
/// # Notes
///
/// This wrapper is handled internally by the client and is transparent
/// to users in most cases. Endpoint methods return the inner `T` directly.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQLResponse<T> {
    pub data: T,
}

/// Pagination wrapper for list responses.
///
/// This struct wraps paginated list responses from the AniList API.
/// It contains pagination metadata and the actual data list.
///
/// # Type Parameters
///
/// * `T` - The type of data list (typically `Vec<SomeType>`)
///
/// # Fields
///
/// * `page_info` - Optional pagination metadata (current page, total, has_next_page, etc.)
/// * `data` - The actual list of items returned by the query
///
/// # Examples
///
/// ```rust
/// # use anilist_moe::AniListClient;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = AniListClient::new();
///
/// // Returns Page<Vec<Media>>
/// let response = client.anime().get_trending_anime(Some(1), Some(10)).await?;
///
/// // Access the Vec<Media> through response.data
/// for anime in &response.data {
///     println!("Title: {:?}", anime.title);
/// }
///
/// // Access pagination info
/// if let Some(page_info) = &response.page_info {
///     println!("Current page: {:?}", page_info.current_page);
///     println!("Has next page: {:?}", page_info.has_next_page);
/// }
/// # Ok(())
/// # }
/// ```
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    /// Pagination metadata including current page, total items, and whether there are more pages
    pub page_info: Option<PageInfo>,
    /// The list of items for this page
    pub data: T,
}

// The high-performance custom deserialization logic
impl<'de, T> Deserialize<'de> for Page<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Define a visitor struct to hold the deserialization logic.
        struct PageVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for PageVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = Page<T>; // The final type we want to produce

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an object with a 'pageInfo' field and one other data field")
            }

            // This method is called by Serde when it encounters a JSON object.
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut page_info = None;
                let mut data = None;

                // Loop through each key-value pair in the JSON object directly.
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "pageInfo" => {
                            // Avoid processing the same field twice.
                            if page_info.is_some() {
                                return Err(de::Error::duplicate_field("pageInfo"));
                            }
                            // Deserialize the value directly into the PageInfo struct.
                            page_info = Some(map.next_value()?);
                        }
                        "extensions" => {
                            // Ignore extensions field if present
                            let _ = map.next_value::<de::IgnoredAny>()?;
                        }
                        // Any other key is treated as the data field.
                        _ => {
                            if data.is_some() {
                                return Err(de::Error::custom("found more than one data field"));
                            }
                            // Deserialize the value directly into the generic type T.
                            data = Some(map.next_value()?);
                        }
                    }
                }

                let data = data.ok_or_else(|| de::Error::custom("missing data field"))?;

                // Construct the final Page struct.
                Ok(Page { page_info, data })
            }
        }

        // Instruct Serde to use our custom visitor to deserialize the object.
        deserializer.deserialize_map(PageVisitor(PhantomData))
    }
}

// A private helper struct to represent the inner object: {"any_key": [...]}.
// It will have its own custom deserialization logic.
struct InnerData<T> {
    data: T,
}

// First, we teach our helper struct how to deserialize itself from an
// object with a single, dynamically named field.
impl<'de, T> Deserialize<'de> for InnerData<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct InnerVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for InnerVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = InnerData<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an object with a single field containing an array")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                // 1. Ignore the key of the single inner field.
                map.next_key::<de::IgnoredAny>()?.ok_or_else(|| {
                    de::Error::custom("expected a single data field, but the object was empty")
                })?;

                // 2. Deserialize the value directly into our final T.
                let data: T = map.next_value()?;

                Ok(InnerData { data })
            }
        }

        deserializer.deserialize_map(InnerVisitor(PhantomData))
    }
}

// Now, we implement Deserialize for our main GraphQLResponse struct.
impl<'de, T> Deserialize<'de> for GraphQLResponse<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OuterVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for OuterVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = GraphQLResponse<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an object with a single 'data' field")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                // 1. Expect the outer key to be "data".
                let key = map.next_key::<String>()?.ok_or_else(|| {
                    de::Error::custom("expected field 'data', but object was empty")
                })?;
                if key != "data" {
                    return Err(de::Error::invalid_value(
                        de::Unexpected::Str(&key),
                        &"the field 'data'",
                    ));
                }

                // 2. Deserialize the value using the custom logic we defined for InnerData.
                let inner: InnerData<T> = map.next_value()?;

                // 3. Construct the final ApiResponse.
                Ok(GraphQLResponse { data: inner.data })
            }
        }

        deserializer.deserialize_map(OuterVisitor(PhantomData))
    }
}
