//! Value Object for the last-update timestamp of a [`UrlSource`].
//!
//! [`UrlSource`]: crate::url_source::domain::entities::url_source::UrlSource

use std::time::SystemTime;

/// An immutable Value Object wrapping a [`SystemTime`] that records when a
/// [`UrlSource`](crate::url_source::domain::entities::url_source::UrlSource)
/// was last updated.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UrlSourceUpdatedAt(SystemTime);

impl UrlSourceUpdatedAt {
    /// Creates a new `UrlSourceUpdatedAt` with the current system time.
    pub fn now() -> Self {
        Self(SystemTime::now())
    }

    /// Creates a `UrlSourceUpdatedAt` from an explicit [`SystemTime`].
    pub fn from_system_time(value: SystemTime) -> Self {
        Self(value)
    }

    /// Returns the underlying [`SystemTime`].
    pub fn value(&self) -> SystemTime {
        self.0
    }
}
