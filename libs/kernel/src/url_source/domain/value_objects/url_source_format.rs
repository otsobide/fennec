//! Value Object for the payload format of a [`UrlSource`].
//!
//! [`UrlSource`]: crate::url_source::domain::entities::url_source::UrlSource

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// An immutable Value Object representing the payload format expected from a
/// [`UrlSource`](crate::url_source::domain::entities::url_source::UrlSource).
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum UrlSourceFormat {
    /// Plain text, one indicator per line.
    Plain,
    /// CSV — comma-separated values.
    Csv,
    /// JSON document.
    Json,
    /// STIX 2.x bundle.
    Stix,
}

impl UrlSourceFormat {
    /// Parses a raw string into a `UrlSourceFormat`, ignoring surrounding whitespace
    /// and letter case.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value is not a recognised
    /// format.
    pub fn from_str(value: &str) -> Result<Self, ValueObjectValidationError> {
        match value.trim().to_ascii_lowercase().as_str() {
            "plain" => Ok(Self::Plain),
            "csv" => Ok(Self::Csv),
            "json" => Ok(Self::Json),
            "stix" => Ok(Self::Stix),
            other => Err(ValueObjectValidationError::new(format!(
                "invalid url source format: {other}"
            ))),
        }
    }

    /// Returns the canonical string representation of this format.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::Csv => "csv",
            Self::Json => "json",
            Self::Stix => "stix",
        }
    }
}

impl std::fmt::Display for UrlSourceFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
