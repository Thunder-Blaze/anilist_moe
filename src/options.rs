use serde::{Serialize, Serializer};
use std::borrow::Cow;

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
