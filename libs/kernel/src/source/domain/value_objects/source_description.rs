//! Value Object for the human-readable description of a [`Source`].
//!
//! [`Source`]: crate::source::domain::entities::source::Source

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// Maximum length, in characters, accepted by [`SourceDescription`].
pub const MAX_LENGTH: usize = 1024;

/// An immutable Value Object wrapping a `String` that holds the description
/// of a [`Source`](crate::source::domain::entities::source::Source).
///
/// Validated at construction: trimmed, and at most [`MAX_LENGTH`] characters.
/// It may be empty: a source is not required to carry a description.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SourceDescription(String);

impl SourceDescription {
    /// Creates a new `SourceDescription` from a raw string, trimming surrounding
    /// whitespace.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] when an invariant is violated.
    pub fn new(value: impl Into<String>) -> Result<Self, ValueObjectValidationError> {
        let value = value.into().trim().to_string();

        if value.chars().count() > MAX_LENGTH {
            return Err(ValueObjectValidationError::new(format!(
                "source description must be at most {MAX_LENGTH} characters, got {}",
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

impl std::fmt::Display for SourceDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
