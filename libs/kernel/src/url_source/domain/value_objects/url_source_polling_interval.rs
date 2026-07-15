//! Value Object for the polling interval (in seconds) of a [`UrlSource`].
//!
//! [`UrlSource`]: crate::url_source::domain::entities::url_source::UrlSource

use thiserror::Error;

/// Errors returned when constructing a [`UrlSourcePollingInterval`].
#[derive(Debug, Error, PartialEq, Eq)]
pub enum UrlSourcePollingIntervalError {
    /// The polling interval is zero. It must be strictly positive.
    #[error("polling interval must be > 0 seconds")]
    Zero,
}

/// An immutable Value Object representing how often (in seconds) a
/// [`UrlSource`](crate::url_source::domain::entities::url_source::UrlSource)
/// is polled.
///
/// Validated at construction: must be strictly positive.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct UrlSourcePollingInterval(u32);

impl UrlSourcePollingInterval {
    /// Creates a new `UrlSourcePollingInterval` from a raw value in seconds.
    ///
    /// # Errors
    ///
    /// Returns [`UrlSourcePollingIntervalError::Zero`] if `seconds` is `0`.
    pub fn from_seconds(seconds: u32) -> Result<Self, UrlSourcePollingIntervalError> {
        if seconds == 0 {
            return Err(UrlSourcePollingIntervalError::Zero);
        }
        Ok(Self(seconds))
    }

    /// Returns the underlying value, in seconds.
    pub fn seconds(&self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for UrlSourcePollingInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}s", self.0)
    }
}
