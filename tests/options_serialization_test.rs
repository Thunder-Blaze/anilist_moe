//! Offline tests for what the options structs put on the wire.
//!
//! The sort fields are the interesting part: every query has to be sorted by
//! something unique, or paging can repeat or skip rows.

use anilist_moe::endpoints::activity::FetchActivityOptions;
use anilist_moe::endpoints::airing::FetchAiringOptions;
use anilist_moe::endpoints::character::FetchCharacterOptions;
use anilist_moe::endpoints::media::FetchMediaOptions;
use anilist_moe::enums::media::{MediaSort, MediaType};
use anilist_moe::options;

/// The serialised form of an options value.
fn wire<T: serde::Serialize>(options: &T) -> serde_json::Value {
    serde_json::to_value(options).expect("options serialise")
}

#[test]
fn an_unset_sort_asks_for_the_id() {
    let options = FetchMediaOptions::default();
    assert_eq!(
        wire(&options)["sort"],
        serde_json::json!(["ID"]),
        "with no sort, the order has to be something unique"
    );
}

#[test]
fn the_id_is_appended_to_a_given_sort() {
    let options =
        options!(FetchMediaOptions, media_type: MediaType::Anime, sort: &[MediaSort::ScoreDesc]);
    assert_eq!(
        wire(&options)["sort"],
        serde_json::json!(["SCORE_DESC", "ID"])
    );

    let two_keys = options!(
        FetchMediaOptions,
        sort: &[MediaSort::ScoreDesc, MediaSort::StartDateDesc]
    );
    assert_eq!(
        wire(&two_keys)["sort"],
        serde_json::json!(["SCORE_DESC", "START_DATE_DESC", "ID"]),
        "the id goes last, after every key the caller asked for"
    );
}

#[test]
fn a_sort_that_already_ends_in_the_id_is_not_duplicated() {
    let options = options!(FetchMediaOptions, sort: &[MediaSort::PopularityDesc, MediaSort::Id]);
    assert_eq!(
        wire(&options)["sort"],
        serde_json::json!(["POPULARITY_DESC", "ID"])
    );
}

#[test]
fn other_fields_are_still_left_out_when_unset() {
    let options = options!(FetchMediaOptions, search: "Naruto");
    let wire = wire(&options);

    assert_eq!(wire["sort"], serde_json::json!(["ID"]));
    assert_eq!(wire["search"], "Naruto");
    assert!(
        wire.get("perPage").is_none(),
        "an unset field must stay off the payload: {wire}"
    );
    assert!(wire.get("includeCharacters").is_none());
}

#[test]
fn an_endpoint_with_a_meaningful_document_default_keeps_it() {
    // These two queries declare their own sort default, and both are total:
    // the activity feed asks for `[ID_DESC]` and the airing schedule `[TIME, ID]`.
    // Sending a sort of our own would replace that order.
    let activity = wire(&FetchActivityOptions::default());
    assert!(
        activity.get("sort").is_none(),
        "the activity feed stays newest first: {activity}"
    );

    let airing = wire(&FetchAiringOptions::default());
    assert!(airing.get("sort").is_none());

    // A sort that is given is still made total.
    let airing =
        options!(FetchAiringOptions, sort: &[anilist_moe::enums::airing::AiringSort::Time]);
    assert_eq!(wire(&airing)["sort"], serde_json::json!(["TIME", "ID"]));
}

#[test]
fn every_endpoint_that_sorts_on_its_own_is_total() {
    let character = wire(&FetchCharacterOptions::default());
    assert_eq!(character["sort"], serde_json::json!(["ID"]));

    let searched = wire(&options!(FetchCharacterOptions, search: "Spike"));
    assert_eq!(searched["sort"], serde_json::json!(["ID"]));
}

// ---------------------------------------------------------------- the contract
//
// AniList validates the *argument* names in a query, not the variable names, so
// a field whose serialised name does not match the variable the query declares
// is dropped without a word. That is exactly what happened to the filter
// suffixes (`id_not`, `status_not_in`, ...), so each group is pinned here.

use anilist_moe::endpoints::activity::{FetchActivityReplyOptions, SubscribeActivityOptions};
use anilist_moe::endpoints::common::{
    LikesOptions, MarkdownOptions, UpdateAniChartHighlightsOptions, UpdateAniChartSettingsOptions,
};
use anilist_moe::endpoints::forum::FetchThreadOptions;
use anilist_moe::endpoints::medialist::FetchMediaListOptions;
use anilist_moe::endpoints::user::{FetchUserMediaListOptions, FetchUserStatsOptions};
use anilist_moe::enums::likable::LikeableType;
use anilist_moe::enums::media::{CountryCode, MediaStatus};
use anilist_moe::enums::media_list::MediaListStatus;
use anilist_moe::objects::anichart::AniChartHighlight;
use std::borrow::Cow;

/// The field's value reaches the API only if the query declares the variable.
fn assert_wired(options: &impl serde::Serialize, keys: &[&str], query: &str) {
    let wire = wire(options);
    for key in keys {
        assert!(
            wire.get(key).is_some(),
            "{key} is missing from the variables"
        );
        assert!(
            query.contains(&format!("${key}")),
            "the query does not declare ${key}"
        );
    }
}

#[test]
fn the_media_filter_suffixes_reach_the_query() {
    let options = FetchMediaOptions {
        id_not: Some(1),
        id_in: Some(Cow::Borrowed(&[1, 2])),
        format_not_in: Some(Cow::Borrowed(&[anilist_moe::enums::media::MediaFormat::Tv])),
        status_not_in: Some(Cow::Borrowed(&[MediaStatus::Finished])),
        genre_in: Some(Cow::Borrowed(&["Action"])),
        country_of_origin_in: Some(Cow::Borrowed(&[CountryCode::Japan])),
        ..Default::default()
    };
    assert_wired(
        &options,
        &[
            "idNot",
            "idIn",
            "formatNotIn",
            "statusNotIn",
            "genreIn",
            "countryOfOriginIn",
        ],
        anilist_moe::queries::media::FETCH,
    );
}

#[test]
fn the_other_filter_suffixes_reach_their_queries() {
    assert_wired(
        &FetchCharacterOptions {
            id_in: Some(Cow::Borrowed(&[1])),
            ..Default::default()
        },
        &["idIn"],
        anilist_moe::queries::character::FETCH,
    );
    assert_wired(
        &FetchThreadOptions {
            id_in: Some(Cow::Borrowed(&[1])),
            ..Default::default()
        },
        &["idIn"],
        anilist_moe::queries::forum::FETCH,
    );
    assert_wired(
        &FetchUserMediaListOptions {
            username: Some(Cow::Borrowed("a")),
            notes: Some(Cow::Borrowed("b")),
            started_at: Some(Cow::Borrowed("c")),
            completed_at: Some(Cow::Borrowed("d")),
            media_id_in: Some(Cow::Borrowed(&[1])),
            status_in: Some(Cow::Borrowed(&[MediaListStatus::Current])),
            ..Default::default()
        },
        &[
            "username",
            "notes",
            "startedAt",
            "completedAt",
            "mediaIdIn",
            "statusIn",
        ],
        anilist_moe::queries::user::MEDIA_LIST,
    );
    assert_wired(
        &SubscribeActivityOptions {
            activity_id: 1,
            subscribe: true,
        },
        &["activityId"],
        anilist_moe::queries::activity::SUBSCRIBE,
    );
}

#[test]
fn a_date_filter_is_the_int_the_api_asks_for() {
    // `startedAt` on a media list is a `FuzzyDateInt` — `YYYYMMDD` — not the
    // `FuzzyDate` object the media objects carry.
    let options = FetchMediaListOptions {
        started_at: Some(20200101),
        ..Default::default()
    };
    assert_eq!(wire(&options)["startedAt"], serde_json::json!(20200101));
    assert!(anilist_moe::queries::medialist::FETCH.contains("$startedAt"));
}

#[test]
fn the_user_lookup_variables_reach_the_query() {
    assert_wired(
        &FetchUserStatsOptions {
            user_id: 1,
            sort: Some(Cow::Borrowed(&[
                anilist_moe::enums::user::UserStatisticsSort::CountDesc,
            ])),
        },
        &["userId", "sort"],
        anilist_moe::queries::user::STATS,
    );
}

#[test]
fn the_new_endpoint_options_serialise() {
    assert_eq!(
        wire(&LikesOptions {
            likeable_id: 7,
            like_type: LikeableType::Activity,
            page: None,
            per_page: Some(5),
        })["likeableId"],
        serde_json::json!(7)
    );
    assert_eq!(
        wire(&FetchActivityReplyOptions {
            id: Some(3),
            activity_id: Some(4),
            text_as_html: Some(true),
        })["activityId"],
        serde_json::json!(4)
    );
}

#[test]
fn the_anichart_and_markdown_options_serialise() {
    assert_eq!(
        wire(&MarkdownOptions {
            markdown: Cow::Borrowed("# Hi"),
        })["markdown"],
        serde_json::json!("# Hi")
    );
    assert_eq!(
        wire(&UpdateAniChartSettingsOptions {
            title_language: Some(Cow::Borrowed("romaji")),
            outgoing_link_provider: None,
            theme: Some(Cow::Borrowed("light")),
            sort: None,
        })["titleLanguage"],
        serde_json::json!("romaji")
    );
    let highlights = wire(&UpdateAniChartHighlightsOptions {
        highlights: Some(Cow::Borrowed(&[AniChartHighlight {
            media_id: Some(16498),
            highlight: Some("watched".to_string()),
        }])),
    });
    assert_eq!(
        highlights["highlights"][0]["mediaId"],
        serde_json::json!(16498)
    );
    assert_eq!(
        highlights["highlights"][0]["highlight"],
        serde_json::json!("watched")
    );
    assert!(anilist_moe::queries::common::UPDATE_ANI_CHART_HIGHLIGHTS.contains("$highlights"));
}
