//! Value Object for the unique identifier of a [`Source`].
//!
//! [`Source`]: crate::source::domain::entities::source::Source

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
use uuid::Uuid;

/// An immutable Value Object wrapping a UUID v4 that uniquely identifies a source.
///
/// Validated at construction: the value must parse as a UUID and be version 4.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SourceId(Uuid);

impl SourceId {
    /// Generates a fresh, platform-side identifier.
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }

    /// Parses a raw string into a `SourceId`.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value is not a valid
    /// UUID, or if it is not version 4.
    pub fn new(value: &str) -> Result<Self, ValueObjectValidationError> {
        let parsed = Uuid::parse_str(value.trim()).map_err(|_| {
            ValueObjectValidationError::new(format!("source id is not a valid UUID: {value}"))
        })?;

        Self::from_uuid(parsed)
    }

    /// Wraps an already-parsed [`Uuid`].
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the UUID is not version 4.
    pub fn from_uuid(value: Uuid) -> Result<Self, ValueObjectValidationError> {
        if value.get_version_num() != 4 {
            return Err(ValueObjectValidationError::new(format!(
                "source id must be a UUID v4, got version {}",
                value.get_version_num()
            )));
        }

        Ok(Self(value))
    }

    /// Returns a reference to the underlying UUID.
    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl std::fmt::Display for SourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
