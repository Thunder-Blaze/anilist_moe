//! Helpers for building endpoint options without boilerplate.
//!
//! Every `*Options` struct takes its string and list fields through
//! [`Cow`](std::borrow::Cow), so a call can pass borrowed literals (`"Naruto"`,
//! `&[MediaSort::IdDesc]`) without allocating, or owned data (`String`, `Vec<T>`)
//! when the options value has to outlive the borrow, as it does when it is moved
//! into a spawned task.
//!
//! [`options!`](crate::options) fills in the `Some(..)` wrappers, so a call site
//! only names the fields it sets:
//!
//! ```
//! use anilist_moe::endpoints::media::FetchMediaOptions;
//! use anilist_moe::enums::media::{MediaSort, MediaType};
//! use anilist_moe::options;
//!
//! // borrowed, zero allocation
//! let borrowed = options!(
//!     FetchMediaOptions,
//!     media_type: MediaType::Anime,
//!     search: "Naruto",
//!     sort: &[MediaSort::IdDesc],
//!     per_page: 5,
//! );
//!
//! // owned data from the caller: same shape, and the value is `'static`
//! let search = String::from("Bleach");
//! let owned = options!(FetchMediaOptions, search: search, per_page: 5);
//! # let _ = (borrowed, owned);
//! ```
//!
//! # Fields
//!
//! * `field: value` wraps the value in `Some(..)`, converting strings and slices
//!   into the [`Cow`](std::borrow::Cow) the field expects.
//! * `field` alone uses the local variable of the same name.
//! * `field => value` assigns the value as it is, for the few fields that are not
//!   optional (`SaveTextActivityOptions::text`, `FetchActivityOneOptions::id`, …).
//! * Fields that are left out keep their default, which is `None`.
//!
//! Both `search: "Naruto"` and `search: Some("Naruto")` work, as do owned values
//! and `Option` bindings, so the macro is also a drop-in for a struct literal.
//!
//! # Sorts
//!
//! Every `sort` field is serialised with the row's unique key appended, through
//! [`SortSpec`] and [`serialize_sort`]. That is what keeps a paginated walk from
//! repeating or skipping rows, and an endpoint left unsorted is sent as the
//! tiebreaker alone.

use serde::{Serialize, Serializer};
use std::borrow::Cow;

/// Converts a value into the `Option<T>` that an options field expects.
///
/// This is what [`options!`](crate::options) calls for every field written as
/// `field: value` or as a lone `field`. It is implemented for:
///
/// * every `T`, producing `Some(value)`
/// * `Option<T>`, passed through unchanged
/// * `&str`, `String` and the same two wrapped in `Option`, when the field is a
///   `Cow<str>`
/// * `&[T]`, `&[T; N]`, `[T; N]`, `Vec<T>` and the same wrapped in `Option`, when
///   the field is a `Cow<[T]>`
///
/// The borrowed forms do not allocate; the owned ones move their data, which
/// keeps the options value `'static`.
///
/// # Why this is not `Into`
///
/// The conversions from `&str` to `Cow<str>` are already `Into` impls; what
/// cannot be written with the standard traits is the next step up — taking
/// `Option<&str>` and `Option<String>` to `Option<Cow<str>>` — because a
/// blanket `impl From<Option<T>> for Option<U>` is forbidden by the orphan rules,
/// as is `impl From<String> for Cow<str>`. A local trait is how the field can
/// accept both the wrapped and the unwrapped form, which is what lets
/// `search: "Naruto"` and `search: Some("Naruto")` mean the same thing.
pub trait IntoOption<T> {
    /// Wraps or converts `self` into an `Option<T>`.
    fn into_opt(self) -> Option<T>;
}

impl<T> IntoOption<T> for T {
    fn into_opt(self) -> Option<T> {
        Some(self)
    }
}

impl<T> IntoOption<T> for Option<T> {
    fn into_opt(self) -> Option<T> {
        self
    }
}

/// Converts a value into the type of a field that is not optional.
///
/// Used by [`options!`](crate::options) for `field => value` entries. The
/// reflexive implementation covers plain fields (`i32`, `bool`, enums) and the
/// string/slice implementations build a [`Cow`] from borrowed or owned data.
///
/// The standard [`Into`] already covers `&str`, `String`, `&[T]` and `Vec<T>` into
/// a `Cow`; this trait exists for the array forms (`[T; N]`, `&[T; N]`), which
/// have no standard conversion, and so that both entry forms of the macro go
/// through one kind of conversion.
pub trait IntoValue<T> {
    /// Converts `self` into `T`.
    fn into_value(self) -> T;
}

impl<T> IntoValue<T> for T {
    fn into_value(self) -> T {
        self
    }
}

impl<'a> IntoValue<Cow<'a, str>> for &'a str {
    fn into_value(self) -> Cow<'a, str> {
        Cow::Borrowed(self)
    }
}

impl<'a> IntoValue<Cow<'a, str>> for String {
    fn into_value(self) -> Cow<'a, str> {
        Cow::Owned(self)
    }
}

impl<'a, T: Clone> IntoValue<Cow<'a, [T]>> for &'a [T] {
    fn into_value(self) -> Cow<'a, [T]> {
        Cow::Borrowed(self)
    }
}

impl<'a, T: Clone, const N: usize> IntoValue<Cow<'a, [T]>> for &'a [T; N] {
    fn into_value(self) -> Cow<'a, [T]> {
        Cow::Borrowed(&self[..])
    }
}

impl<'a, T: Clone> IntoValue<Cow<'a, [T]>> for Vec<T> {
    fn into_value(self) -> Cow<'a, [T]> {
        Cow::Owned(self)
    }
}

impl<'a, T: Clone, const N: usize> IntoValue<Cow<'a, [T]>> for [T; N] {
    fn into_value(self) -> Cow<'a, [T]> {
        Cow::Owned(self.to_vec())
    }
}

impl<'a> IntoOption<Cow<'a, str>> for &'a str {
    fn into_opt(self) -> Option<Cow<'a, str>> {
        Some(Cow::Borrowed(self))
    }
}

impl<'a> IntoOption<Cow<'a, str>> for String {
    fn into_opt(self) -> Option<Cow<'a, str>> {
        Some(Cow::Owned(self))
    }
}

impl<'a> IntoOption<Cow<'a, str>> for Option<&'a str> {
    fn into_opt(self) -> Option<Cow<'a, str>> {
        self.map(Cow::Borrowed)
    }
}

impl<'a> IntoOption<Cow<'a, str>> for Option<String> {
    fn into_opt(self) -> Option<Cow<'a, str>> {
        self.map(Cow::Owned)
    }
}

impl<'a, T: Clone> IntoOption<Cow<'a, [T]>> for &'a [T] {
    fn into_opt(self) -> Option<Cow<'a, [T]>> {
        Some(Cow::Borrowed(self))
    }
}

impl<'a, T: Clone, const N: usize> IntoOption<Cow<'a, [T]>> for &'a [T; N] {
    fn into_opt(self) -> Option<Cow<'a, [T]>> {
        Some(Cow::Borrowed(&self[..]))
    }
}

impl<'a, T: Clone> IntoOption<Cow<'a, [T]>> for Vec<T> {
    fn into_opt(self) -> Option<Cow<'a, [T]>> {
        Some(Cow::Owned(self))
    }
}

impl<'a, T: Clone, const N: usize> IntoOption<Cow<'a, [T]>> for [T; N] {
    fn into_opt(self) -> Option<Cow<'a, [T]>> {
        Some(Cow::Owned(self.to_vec()))
    }
}

impl<'a, T: Clone> IntoOption<Cow<'a, [T]>> for Option<&'a [T]> {
    fn into_opt(self) -> Option<Cow<'a, [T]>> {
        self.map(Cow::Borrowed)
    }
}

impl<'a, T: Clone, const N: usize> IntoOption<Cow<'a, [T]>> for Option<&'a [T; N]> {
    fn into_opt(self) -> Option<Cow<'a, [T]>> {
        self.map(|s| Cow::Borrowed(&s[..]))
    }
}

impl<'a, T: Clone> IntoOption<Cow<'a, [T]>> for Option<Vec<T>> {
    fn into_opt(self) -> Option<Cow<'a, [T]>> {
        self.map(Cow::Owned)
    }
}

impl<'a, T: Clone, const N: usize> IntoOption<Cow<'a, [T]>> for Option<[T; N]> {
    fn into_opt(self) -> Option<Cow<'a, [T]>> {
        self.map(|s| Cow::Owned(s.to_vec()))
    }
}

/// Builds an options value, filling in `Some(..)` and the remaining defaults.
///
/// The first argument is the options type, which has to be a single identifier
/// already in scope; the second is the list of fields to set, in any order:
///
/// ```
/// use anilist_moe::endpoints::media::FetchMediaOptions;
/// use anilist_moe::enums::media::MediaSort;
/// use anilist_moe::options;
///
/// let opts = options!(
///     FetchMediaOptions,
///     search: "Naruto",
///     sort: &[MediaSort::IdDesc],
///     per_page: 5,
/// );
/// # let _ = opts;
/// ```
///
/// `field: value` wraps the value in `Some(..)`, a lone `field` uses the local
/// variable of the same name, and `field => value` assigns the value unchanged
/// for the fields that are not optional. See the [module docs](crate::options)
/// for the accepted value types.
#[macro_export]
macro_rules! options {
    ($t:ident $(, $field:ident $(: $value:expr)? $(=> $plain:expr)?)* $(,)?) => {{
        // `..Default::default()` fills in the fields that were not named, which
        // clippy flags as needless when a call happens to set every field.
        #[allow(clippy::needless_update)]
        let options = $t {
            $( $field: $crate::options!(@field $field $(: $value)? $(=> $plain)?), )*
            ..Default::default()
        };
        options
    }};
    (@field $field:ident) => {
        $crate::options::IntoOption::into_opt($field)
    };
    (@field $field:ident : $value:expr) => {
        $crate::options::IntoOption::into_opt($value)
    };
    (@field $field:ident => $plain:expr) => {
        $crate::options::IntoValue::into_value($plain)
    };
}

/// A sort key that has a unique field to break ties with.
///
/// AniList sorts on the database columns as they are, so two rows that share the
/// sort value may come back in either order, and an order that changes between
/// requests makes a paginated walk repeat or skip rows. Every sort field is
/// therefore serialised with its unique key appended, which is what this trait
/// supplies.
///
/// ```rust
/// use anilist_moe::enums::media::MediaSort;
/// use anilist_moe::options::SortSpec;
///
/// assert_eq!(MediaSort::id_tiebreaker(), MediaSort::Id);
/// ```
pub trait SortSpec: Sized {
    /// The key that is unique per row, used as the last sort field.
    ///
    /// It is the row id — `MediaId` for a media list entry, which has no `Id`.
    fn id_tiebreaker() -> Self;
}

/// Appends [`SortSpec::id_tiebreaker`] to `sort`, unless it is already there.
///
/// `None` and an empty list both become just the tiebreaker, so a query is
/// always sorted by something total.
#[must_use]
pub fn with_id_tiebreaker<T>(sort: Option<&[T]>) -> Vec<T>
where
    T: SortSpec + PartialEq + Clone,
{
    let tiebreaker = T::id_tiebreaker();
    let mut keys: Vec<T> = sort.unwrap_or(&[]).to_vec();

    if !keys.contains(&tiebreaker) {
        keys.push(tiebreaker);
    }

    keys
}

/// Serialises a sort list with its tiebreaker appended.
///
/// Used by the `sort` field of every options struct. An unset sort becomes the
/// tiebreaker alone, which is the deterministic replacement for the implicit
/// order the server would otherwise pick.
///
/// # Errors
///
/// Whatever the underlying serializer reports.
pub fn serialize_sort<T, S>(sort: &Option<Cow<'_, [T]>>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Serialize + SortSpec + PartialEq + Clone,
    S: Serializer,
{
    let keys = with_id_tiebreaker(sort.as_deref());
    serializer.collect_seq(keys)
}

/// A `skip_serializing_if` predicate that never skips.
///
/// The sort fields use it so that an unset sort still reaches the wire; every
/// other optional field keeps the `skip_serializing_none` behaviour.
#[must_use]
pub fn skip_never<T>(_: &T) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::media::MediaSort;
    use crate::enums::media_list::MediaListSort;

    #[test]
    fn an_unset_sort_becomes_the_tiebreaker() {
        assert_eq!(with_id_tiebreaker::<MediaSort>(None), vec![MediaSort::Id]);
        assert_eq!(
            with_id_tiebreaker::<MediaSort>(Some(&[])),
            vec![MediaSort::Id]
        );
    }

    #[test]
    fn the_tiebreaker_is_appended_after_the_given_keys() {
        assert_eq!(
            with_id_tiebreaker(Some(&[MediaSort::ScoreDesc])),
            vec![MediaSort::ScoreDesc, MediaSort::Id]
        );
        assert_eq!(
            with_id_tiebreaker(Some(&[MediaSort::ScoreDesc, MediaSort::StartDateDesc])),
            vec![
                MediaSort::ScoreDesc,
                MediaSort::StartDateDesc,
                MediaSort::Id
            ],
            "the id goes last, never in the middle"
        );
    }

    #[test]
    fn a_sort_that_already_ends_in_the_tiebreaker_is_left_alone() {
        assert_eq!(
            with_id_tiebreaker(Some(&[MediaSort::ScoreDesc, MediaSort::Id])),
            vec![MediaSort::ScoreDesc, MediaSort::Id],
            "no duplicate key"
        );
        assert_eq!(
            with_id_tiebreaker(Some(&[MediaSort::Id])),
            vec![MediaSort::Id]
        );
    }

    #[test]
    fn a_media_list_entry_is_tied_by_its_media_id() {
        assert_eq!(
            with_id_tiebreaker(Some(&[MediaListSort::ScoreDesc])),
            vec![MediaListSort::ScoreDesc, MediaListSort::MediaId],
            "a media list entry has no Id of its own"
        );
    }
}
