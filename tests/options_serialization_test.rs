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
