//! Value Object for the unique identifier of a [`Sighting`].
//!
//! [`Sighting`]: crate::sighting::domain::entities::sighting::Sighting

use uuid::Uuid;

/// An immutable Value Object wrapping a UUID v4 that uniquely identifies a
/// [`Sighting`](crate::sighting::domain::entities::sighting::Sighting).
///
/// The identifier is **externally provided** at construction time.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SightingId(Uuid);

impl SightingId {
    /// Creates a new `SightingId` from a UUID supplied by the caller.
    pub fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }

    /// Returns a reference to the underlying UUID.
    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl std::fmt::Display for SightingId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
