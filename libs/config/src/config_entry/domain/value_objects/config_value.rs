//! Value Object for the value stored in a config entry.

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// Maximum length, in characters, accepted by [`ConfigValue`].
pub const MAX_LENGTH: usize = 4096;

/// An immutable Value Object wrapping a `String` that holds the content
/// of a [`ConfigEntry`](crate::config_entry::domain::entities::config_entry::ConfigEntry).
///
/// Validated at construction: trimmed and at most [`MAX_LENGTH`] characters.
/// It may be empty: an entry is allowed to store an empty value.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConfigValue(String);

impl ConfigValue {
    /// Creates a new `ConfigValue` from a raw string, trimming surrounding
    /// whitespace.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] when an invariant is violated.
    pub fn new(value: impl Into<String>) -> Result<Self, ValueObjectValidationError> {
        let value = value.into().trim().to_string();

        if value.chars().count() > MAX_LENGTH {
            return Err(ValueObjectValidationError::new(format!(
                "config value must be at most {MAX_LENGTH} characters, got {}",
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

impl std::fmt::Display for ConfigValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
