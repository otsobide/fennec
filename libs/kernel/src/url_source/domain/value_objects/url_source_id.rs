//! Value Object for the unique identifier of a [`UrlSource`].
//!
//! By design, this UUID is shared with the matching
//! [`Source`](https://docs.rs/kernel) aggregate in the `kernel` bounded context:
//! `UrlSource.id == Source.id`. The relationship between the two contexts is
//! purely by identifier — `url_source` does not depend on `kernel` types.
//!
//! [`UrlSource`]: crate::url_source::domain::entities::url_source::UrlSource

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
use uuid::Uuid;

/// An immutable Value Object wrapping a UUID v4 that uniquely identifies a url source.
///
/// Validated at construction: the value must parse as a UUID and be version 4.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct UrlSourceId(Uuid);

impl UrlSourceId {
    /// Generates a fresh, platform-side identifier.
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }

    /// Parses a raw string into a `UrlSourceId`.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value is not a valid
    /// UUID, or if it is not version 4.
    pub fn new(value: &str) -> Result<Self, ValueObjectValidationError> {
        let parsed = Uuid::parse_str(value.trim()).map_err(|_| {
            ValueObjectValidationError::new(format!("url source id is not a valid UUID: {value}"))
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
                "url source id must be a UUID v4, got version {}",
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

impl std::fmt::Display for UrlSourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
