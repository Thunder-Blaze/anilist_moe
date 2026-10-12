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
