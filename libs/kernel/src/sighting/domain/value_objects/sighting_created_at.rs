//! Value Object for the creation timestamp of a [`Sighting`].
//!
//! [`Sighting`]: crate::sighting::domain::entities::sighting::Sighting

use std::time::SystemTime;

use shared_valueobject::domain::timestamps::truncate_to_microseconds::truncate_to_microseconds;

/// An immutable Value Object wrapping a [`SystemTime`] that records when a
/// [`Sighting`](crate::sighting::domain::entities::sighting::Sighting) was
/// created.
///
/// Truncated to whole microseconds, the precision PostgreSQL stores, so it
/// compares equal after a round trip through the database.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SightingCreatedAt(SystemTime);

impl SightingCreatedAt {
    /// Creates a new `SightingCreatedAt` with the current system time.
    pub fn now() -> Self {
        Self(truncate_to_microseconds(SystemTime::now()))
    }

    /// Creates a `SightingCreatedAt` from an explicit [`SystemTime`].
    pub fn from_system_time(value: SystemTime) -> Self {
        Self(truncate_to_microseconds(value))
    }

    /// Returns the underlying [`SystemTime`].
    pub fn value(&self) -> SystemTime {
        self.0
    }
}
