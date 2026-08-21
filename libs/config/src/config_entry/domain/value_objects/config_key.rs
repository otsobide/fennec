//! Value Object for the unique key of a config entry.

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// Maximum length, in characters, accepted by [`ConfigKey`].
pub const MAX_LENGTH: usize = 255;

/// An immutable Value Object wrapping a `String` that uniquely identifies a
/// [`ConfigEntry`](crate::config_entry::domain::entities::config_entry::ConfigEntry).
///
/// Validated at construction: trimmed, non-empty, and at most [`MAX_LENGTH`]
/// characters.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConfigKey(String);

impl ConfigKey {
    /// Creates a new `ConfigKey` from a raw string, trimming surrounding
    /// whitespace.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] when an invariant is violated.
    pub fn new(value: impl Into<String>) -> Result<Self, ValueObjectValidationError> {
        let value = value.into().trim().to_string();

        if value.is_empty() {
            return Err(ValueObjectValidationError::new(
                "config key must not be empty".to_string(),
            ));
        }

        if value.chars().count() > MAX_LENGTH {
            return Err(ValueObjectValidationError::new(format!(
                "config key must be at most {MAX_LENGTH} characters, got {}",
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

impl std::fmt::Display for ConfigKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
