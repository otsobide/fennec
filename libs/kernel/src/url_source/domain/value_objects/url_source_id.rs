//! Value Object for the unique identifier of a [`UrlSource`].
//!
//! By design, this UUID is shared with the matching
//! [`Source`](https://docs.rs/kernel) aggregate in the `kernel` bounded context:
//! `UrlSource.id == Source.id`. The relationship between the two contexts is
//! purely by identifier — `url_source` does not depend on `kernel` types.
//!
//! [`UrlSource`]: crate::url_source::domain::entities::url_source::UrlSource

use uuid::Uuid;

/// An immutable Value Object wrapping a UUID v4 that uniquely identifies a
/// [`UrlSource`](crate::url_source::domain::entities::url_source::UrlSource).
///
/// The identifier is **externally provided** at construction time and must
/// match the id of the matching `Source` aggregate in `kernel`.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct UrlSourceId(Uuid);

impl UrlSourceId {
    /// Creates a new `UrlSourceId` from a UUID supplied by the caller.
    pub fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }

    /// Returns a reference to the underlying UUID.
    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl std::fmt::Display for UrlSourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
