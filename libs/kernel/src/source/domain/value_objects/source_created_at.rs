//! Value Object for the creation timestamp of a [`Source`].
//!
//! [`Source`]: crate::source::domain::entities::source::Source

use std::time::SystemTime;

use shared_valueobject::domain::timestamps::truncate_to_microseconds::truncate_to_microseconds;

/// An immutable Value Object wrapping a [`SystemTime`] that records when a
/// [`Source`](crate::source::domain::entities::source::Source) was created.
///
/// Truncated to whole microseconds, the precision PostgreSQL stores, so it
/// compares equal after a round trip through the database.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SourceCreatedAt(SystemTime);

impl SourceCreatedAt {
    /// Creates a new `SourceCreatedAt` with the current system time.
    pub fn now() -> Self {
        Self(truncate_to_microseconds(SystemTime::now()))
    }

    /// Creates a `SourceCreatedAt` from an explicit [`SystemTime`].
    ///
    /// Useful when reconstituting from a persistent store.
    pub fn from_system_time(value: SystemTime) -> Self {
        Self(truncate_to_microseconds(value))
    }

    /// Returns the underlying [`SystemTime`].
    pub fn value(&self) -> SystemTime {
        self.0
    }
}
