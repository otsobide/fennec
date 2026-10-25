//! Value Object for the type of a [`Source`].
//!
//! [`Source`]: crate::source::domain::entities::source::Source

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// An immutable Value Object representing the type of a
/// [`Source`](crate::source::domain::entities::source::Source).
///
/// Only the variants enumerated here are accepted; constructing one from a
/// raw string goes through [`SourceType::from_str`], which validates the input.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SourceType {
    /// HTTP/HTTPS URL feed.
    Url,
}

impl SourceType {
    /// Parses a raw string into a `SourceType`, ignoring surrounding whitespace
    /// and letter case.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value is not a recognised
    /// source type.
    pub fn from_str(value: &str) -> Result<Self, ValueObjectValidationError> {
        match value.trim().to_ascii_lowercase().as_str() {
            "url" => Ok(Self::Url),
            other => Err(ValueObjectValidationError::new(format!(
                "invalid source type: {other}"
            ))),
        }
    }

    /// Returns the canonical string representation of this source type.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Url => "url",
        }
    }
}

impl std::fmt::Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
