//! Value Object for the first-seen timestamp of a [`Sighting`].
//!
//! [`Sighting`]: crate::sighting::domain::entities::sighting::Sighting

use std::time::SystemTime;

use shared_valueobject::domain::timestamps::truncate_to_microseconds::truncate_to_microseconds;

/// An immutable Value Object wrapping a [`SystemTime`] that records when the
/// referenced observable was first reported by the referenced source.
///
/// The timestamp is externally provided by the caller (typically the ingester
/// via the `observed_at` field of the create command).
///
/// Truncated to whole microseconds, the precision PostgreSQL stores, so it
/// compares equal after a round trip through the database.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SightingFirstSeen(SystemTime);

impl SightingFirstSeen {
    /// Creates a `SightingFirstSeen` from an explicit [`SystemTime`].
    pub fn from_system_time(value: SystemTime) -> Self {
        Self(truncate_to_microseconds(value))
    }

    /// Returns the underlying [`SystemTime`].
    pub fn value(&self) -> SystemTime {
        self.0
    }
}
