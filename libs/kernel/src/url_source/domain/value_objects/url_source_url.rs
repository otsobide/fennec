//! Value Object for the HTTP/HTTPS URL of a [`UrlSource`].
//!
//! [`UrlSource`]: crate::url_source::domain::entities::url_source::UrlSource

use thiserror::Error;

/// Errors returned when constructing a [`UrlSourceUrl`].
#[derive(Debug, Error, PartialEq, Eq)]
pub enum UrlSourceUrlError {
    /// The URL is empty.
    #[error("url is empty")]
    Empty,
    /// The URL does not start with `http://` or `https://`.
    #[error("url must start with http:// or https://")]
    InvalidScheme,
}

/// An immutable Value Object wrapping the URL of a
/// [`UrlSource`](crate::url_source::domain::entities::url_source::UrlSource).
///
/// Validated at construction: must be non-empty and use the `http` or `https`
/// scheme.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UrlSourceUrl(String);

impl UrlSourceUrl {
    /// Creates a new `UrlSourceUrl` from a raw string.
    ///
    /// # Errors
    ///
    /// Returns [`UrlSourceUrlError::Empty`] if the value is empty, or
    /// [`UrlSourceUrlError::InvalidScheme`] if it does not start with
    /// `http://` or `https://`.
    pub fn new(value: impl Into<String>) -> Result<Self, UrlSourceUrlError> {
        let value = value.into();
        if value.is_empty() {
            return Err(UrlSourceUrlError::Empty);
        }
        if !value.starts_with("http://") && !value.starts_with("https://") {
            return Err(UrlSourceUrlError::InvalidScheme);
        }
        Ok(Self(value))
    }

    /// Returns a reference to the underlying URL string.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for UrlSourceUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
