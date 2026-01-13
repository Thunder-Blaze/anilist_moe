//! Custom query usage with `fetch`.
//!
//! Fetches a subset of media fields via a manual GraphQL query.

mod test_harness;

use anilist_moe::objects::{media::Media, responses::Page};
use std::collections::HashMap;
use test_harness::{TestHarness, delay_between_tests};

/// GraphQL query: id + title for up to 5 media
const CUSTOM_MEDIA_QUERY: &str = r#"
query ($page: Int, $perPage: Int) {
  Page(page: $page, perPage: $perPage) {
    media {
      id
      title {
        romaji
        english
        native
      }
    }
  }
}
"#;

#[tokio::test]
async fn test_custom_query_with_fetch() {
    let h = TestHarness::new();
    let client = h.client();

    let result = h
        .run(|| async {
            let mut variables = HashMap::new();
            variables.insert("page".to_string(), serde_json::json!(1));
            variables.insert("perPage".to_string(), serde_json::json!(5));

            client
                .fetch::<Page<Vec<Media>>>(CUSTOM_MEDIA_QUERY, Some(&variables))
                .await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch media with custom query: {:?}",
        result.err()
    );

    let media_list = result.unwrap().data;

    assert!(
        !media_list.is_empty(),
        "Should return at least one media result"
    );
    assert!(
        media_list.len() <= 5,
        "Should respect perPage limit (got {})",
        media_list.len()
    );

    for media in &media_list {
        assert!(media.id.is_some(), "Media should have an id");
        assert!(media.id.unwrap() > 0, "Media id should be positive");
        assert!(media.title.is_some(), "Media should have a title");

        if let Some(ref title) = media.title {
            let has_title =
                title.romaji.is_some() || title.english.is_some() || title.native.is_some();
            assert!(has_title, "At least one title variant should be present");
        }
    }
}

/// Media from custom queries remains compatible
#[tokio::test]
async fn test_custom_query_media_compatibility() {
    delay_between_tests().await;
    let h = TestHarness::new();
    let client = h.client();

    let result = h
        .run(|| async {
            let mut variables = HashMap::new();
            variables.insert("page".to_string(), serde_json::json!(1));
            variables.insert("perPage".to_string(), serde_json::json!(3));

            client
                .fetch::<Page<Vec<Media>>>(CUSTOM_MEDIA_QUERY, Some(&variables))
                .await
        })
        .await;

    assert!(result.is_ok(), "Custom query should succeed");
    let media_list = result.unwrap().data;

    for media in media_list {
        let id = media.id.expect("Media should have id");
        let title = media.title.as_ref().expect("Media should have title");

        // Verify we can access title fields
        let _romaji = title.romaji.as_ref();
        let _english = title.english.as_ref();
        let _native = title.native.as_ref();

        // Verify Media can be cloned and compared
        let _cloned = media.clone();

        // Verify Media implements Debug
        let _debug = format!("{:?}", media);

        // Media objects from custom queries should be fully compatible
        assert!(id > 0, "Media ID should be positive");
    }
}
