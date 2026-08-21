//! Value Object for the source identifier referenced by a [`Sighting`].
//!
//! References a `Source` aggregate by identifier only. The `sighting` bounded
//! context does not import from the `kernel` crate.
//!
//! [`Sighting`]: crate::sighting::domain::entities::sighting::Sighting

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
use uuid::Uuid;

/// An immutable Value Object wrapping a UUID v4 that uniquely identifies a source referenced by a sighting.
///
/// Validated at construction: the value must parse as a UUID and be version 4.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SightingSourceId(Uuid);

impl SightingSourceId {
    /// Generates a fresh, platform-side identifier.
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }

    /// Parses a raw string into a `SightingSourceId`.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value is not a valid
    /// UUID, or if it is not version 4.
    pub fn new(value: &str) -> Result<Self, ValueObjectValidationError> {
        let parsed = Uuid::parse_str(value.trim()).map_err(|_| {
            ValueObjectValidationError::new(format!(
                "sighting source id is not a valid UUID: {value}"
            ))
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
                "sighting source id must be a UUID v4, got version {}",
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

impl std::fmt::Display for SightingSourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
