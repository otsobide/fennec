//! Value Object for the last-seen timestamp of a [`Sighting`].
//!
//! [`Sighting`]: crate::sighting::domain::entities::sighting::Sighting

use std::time::SystemTime;

/// An immutable Value Object wrapping a [`SystemTime`] that records the most
/// recent observation of the referenced observable by the referenced source.
///
/// The timestamp is externally provided by the caller (typically the ingester
/// via the `observed_at` field of the create/observe command).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SightingLastSeen(SystemTime);

impl SightingLastSeen {
    /// Creates a `SightingLastSeen` from an explicit [`SystemTime`].
    pub fn from_system_time(value: SystemTime) -> Self {
        Self(value)
    }

    /// Returns the underlying [`SystemTime`].
    pub fn value(&self) -> SystemTime {
        self.0
    }
}
