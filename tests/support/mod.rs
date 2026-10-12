//! Helpers shared by the live API tests.
//!
//! These tests talk to the real AniList API, so a failure means one of two very
//! different things: the environment let us down (no network, an outage, a rate
//! limit), or the crate is wrong. Only the first may be swallowed. A test that
//! passes because the response was unparseable is worse than no test at all, so
//! everything else fails loudly here.

#![allow(dead_code)]

use anilist_moe::AniListError;

/// Whether `error` is the environment's fault rather than the crate's.
///
/// A `5xx` counts: AniList answers a rejected query with a `400` and a GraphQL
/// error list, so a server error means the server, not the query.
pub fn is_environmental(error: &AniListError) -> bool {
    matches!(
        error,
        AniListError::Network(_)
            | AniListError::Transport { .. }
            | AniListError::Maintenance { .. }
            | AniListError::RateLimit { .. }
            | AniListError::RateLimitSimple
            | AniListError::BurstLimit
            | AniListError::ServerError { .. }
    )
}

fn skip(context: &str, error: &AniListError) {
    eprintln!("SKIP {context}: the environment failed, not the crate: {error}");
}

/// Skips `context` when the environment failed, and fails the test otherwise.
pub fn allow_environmental_failure(context: &str, error: &AniListError) {
    if is_environmental(error) {
        skip(context, error);
        return;
    }
    panic!("{context} failed for a reason that is not environmental: {error:?}");
}

/// Accepts the auth/permission rejection these unauthenticated tests exist to
/// observe — and nothing else, apart from an environment failure.
pub fn allow_auth_rejection(context: &str, error: &AniListError) {
    if is_environmental(error) {
        skip(context, error);
        return;
    }
    if matches!(
        error,
        AniListError::AuthenticationRequired
            | AniListError::AccessDenied
            | AniListError::GraphQL { .. }
            | AniListError::BadRequest { .. }
    ) {
        eprintln!("{context}: the API rejected the request, as this test expects: {error}");
        return;
    }
    panic!("{context} failed for an unexpected reason: {error:?}");
}
