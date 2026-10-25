//! Value Object for the identifier of the observable a [`Sighting`] refers
//! to.
//!
//! [`Sighting`]: crate::sighting::domain::entities::sighting::Sighting

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
use uuid::{Uuid, Variant};

/// An immutable Value Object holding the id of the sighted observable.
///
/// Value-identified observables have UUID v5 ids derived from their value,
/// the others UUID v4 ids, so both versions are accepted. The id means
/// nothing without its [`SightingObservableKind`].
///
/// [`SightingObservableKind`]: crate::sighting::domain::value_objects::sighting_observable_kind::SightingObservableKind
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SightingObservableId(Uuid);

impl SightingObservableId {
    /// Parses a raw string into a `SightingObservableId`.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value is not a valid
    /// UUID, or if it is not an RFC 4122 UUID of version 4 or 5.
    pub fn new(value: &str) -> Result<Self, ValueObjectValidationError> {
        let parsed = Uuid::parse_str(value.trim()).map_err(|_| {
            ValueObjectValidationError::new(format!(
                "sighting observable id is not a valid UUID: {value}"
            ))
        })?;

        Self::from_uuid(parsed)
    }

    /// Wraps an already-parsed [`Uuid`].
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the UUID is not an RFC 4122
    /// UUID of version 4 or 5.
    pub fn from_uuid(value: Uuid) -> Result<Self, ValueObjectValidationError> {
        let version = value.get_version_num();
        if !(version == 4 || version == 5) || value.get_variant() != Variant::RFC4122 {
            return Err(ValueObjectValidationError::new(format!(
                "sighting observable id must be an RFC 4122 UUID v4 or v5, got version {version} with the {:?} variant",
                value.get_variant()
            )));
        }

        Ok(Self(value))
    }

    /// Returns a reference to the underlying UUID.
    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl std::fmt::Display for SightingObservableId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
