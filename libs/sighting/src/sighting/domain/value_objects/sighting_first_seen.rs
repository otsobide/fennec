//! Value Object for the first-seen timestamp of a [`Sighting`].
//!
//! [`Sighting`]: crate::sighting::domain::entities::sighting::Sighting

use std::time::SystemTime;

/// An immutable Value Object wrapping a [`SystemTime`] that records when the
/// referenced observable was first reported by the referenced source.
///
/// The timestamp is externally provided by the caller (typically the ingester
/// via the `observed_at` field of the create command).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SightingFirstSeen(SystemTime);

impl SightingFirstSeen {
    /// Creates a `SightingFirstSeen` from an explicit [`SystemTime`].
    pub fn from_system_time(value: SystemTime) -> Self {
        Self(value)
    }

    /// Returns the underlying [`SystemTime`].
    pub fn value(&self) -> SystemTime {
        self.0
    }
}
