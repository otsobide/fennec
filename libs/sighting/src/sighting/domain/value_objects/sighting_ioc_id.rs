//! Value Object for the ioc identifier referenced by a [`Sighting`].
//!
//! References an `Ioc` aggregate by identifier only. The `sighting` bounded
//! context does not import from the `ioc` crate.
//!
//! [`Sighting`]: crate::sighting::domain::entities::sighting::Sighting

use uuid::Uuid;

/// An immutable Value Object wrapping a UUID v4 that identifies the ioc
/// referenced by a
/// [`Sighting`](crate::sighting::domain::entities::sighting::Sighting).
///
/// The identifier is **externally provided** at construction time.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SightingIocId(Uuid);

impl SightingIocId {
    /// Creates a new `SightingIocId` from a UUID supplied by the caller.
    pub fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }

    /// Returns a reference to the underlying UUID.
    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl std::fmt::Display for SightingIocId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
