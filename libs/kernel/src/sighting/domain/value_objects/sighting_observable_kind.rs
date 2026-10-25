//! Value Object for the kind of observable a [`Sighting`] refers to.
//!
//! [`Sighting`]: crate::sighting::domain::entities::sighting::Sighting

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// Maximum length, in characters, accepted by [`SightingObservableKind`].
pub const MAX_LENGTH: usize = 64;

/// An immutable Value Object naming the observable kind a sighting refers to,
/// as the `kernel` module that owns it is named (`ipv4_address`).
///
/// It is a validated name and not an enum, so adding an observable kind never
/// touches the sighting module. Together with [`SightingObservableId`] it
/// qualifies the reference: ids of different kinds can share a UUID.
///
/// Validated at construction: trimmed, then lowercase ASCII letters, digits
/// and single underscores, starting with a letter, at most [`MAX_LENGTH`]
/// characters.
///
/// [`SightingObservableId`]: crate::sighting::domain::value_objects::sighting_observable_id::SightingObservableId
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SightingObservableKind(String);

impl SightingObservableKind {
    /// Parses an observable kind name.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value is not a snake case
    /// module name.
    pub fn new(value: &str) -> Result<Self, ValueObjectValidationError> {
        let trimmed = value.trim();

        if trimmed.chars().count() > MAX_LENGTH {
            return Err(ValueObjectValidationError::new(format!(
                "sighting observable kind must be at most {MAX_LENGTH} characters, got {}",
                trimmed.chars().count()
            )));
        }

        if !is_snake_case_name(trimmed) {
            return Err(ValueObjectValidationError::new(format!(
                "sighting observable kind must be a snake case module name such as ipv4_address, got {value}"
            )));
        }

        Ok(Self(trimmed.to_string()))
    }

    /// Returns the kind name.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SightingObservableKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn is_snake_case_name(value: &str) -> bool {
    let starts_with_letter = value
        .chars()
        .next()
        .is_some_and(|first| first.is_ascii_lowercase());
    let allowed_characters = value
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');

    starts_with_letter && allowed_characters && !value.ends_with('_') && !value.contains("__")
}
