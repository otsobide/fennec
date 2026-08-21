//! Value Object for the HTTP/HTTPS URL of a [`UrlSource`].
//!
//! [`UrlSource`]: crate::url_source::domain::entities::url_source::UrlSource

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// Maximum length, in characters, accepted by [`UrlSourceUrl`].
pub const MAX_LENGTH: usize = 2048;

/// An immutable Value Object wrapping the URL of a
/// [`UrlSource`](crate::url_source::domain::entities::url_source::UrlSource).
///
/// Validated at construction: trimmed, non-empty, `http`/`https` scheme, and
/// at most [`MAX_LENGTH`] characters.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UrlSourceUrl(String);

impl UrlSourceUrl {
    /// Creates a new `UrlSourceUrl` from a raw string, trimming surrounding
    /// whitespace.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] when an invariant is violated.
    pub fn new(value: impl Into<String>) -> Result<Self, ValueObjectValidationError> {
        let value = value.into().trim().to_string();

        if value.is_empty() {
            return Err(ValueObjectValidationError::new(
                "url source url must not be empty".to_string(),
            ));
        }

        if !value.starts_with("http://") && !value.starts_with("https://") {
            return Err(ValueObjectValidationError::new(
                "url source url must start with http:// or https://".to_string(),
            ));
        }

        if value.chars().count() > MAX_LENGTH {
            return Err(ValueObjectValidationError::new(format!(
                "url source url must be at most {MAX_LENGTH} characters, got {}",
                value.chars().count()
            )));
        }

        Ok(Self(value))
    }

    /// Returns a reference to the underlying string.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for UrlSourceUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
