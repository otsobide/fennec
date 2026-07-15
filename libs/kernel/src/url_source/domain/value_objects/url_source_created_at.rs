//! Value Object for the creation timestamp of a [`UrlSource`].
//!
//! [`UrlSource`]: crate::url_source::domain::entities::url_source::UrlSource

use std::time::SystemTime;

/// An immutable Value Object wrapping a [`SystemTime`] that records when a
/// [`UrlSource`](crate::url_source::domain::entities::url_source::UrlSource)
/// was created.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UrlSourceCreatedAt(SystemTime);

impl UrlSourceCreatedAt {
    /// Creates a new `UrlSourceCreatedAt` with the current system time.
    pub fn now() -> Self {
        Self(SystemTime::now())
    }

    /// Creates a `UrlSourceCreatedAt` from an explicit [`SystemTime`].
    pub fn from_system_time(value: SystemTime) -> Self {
        Self(value)
    }

    /// Returns the underlying [`SystemTime`].
    pub fn value(&self) -> SystemTime {
        self.0
    }
}
