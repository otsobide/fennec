//! Value Object for the raw observable value of an [`Ioc`].
//!
//! [`Ioc`]: crate::ioc::domain::entities::ioc::Ioc

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// Maximum length, in characters, accepted by [`IocValue`].
pub const MAX_LENGTH: usize = 2048;

/// An immutable Value Object wrapping the observable string of an
/// [`Ioc`](crate::ioc::domain::entities::ioc::Ioc).
///
/// Validated at construction: trimmed, non-empty, and at most [`MAX_LENGTH`]
/// characters. Per-type validation (IPv4 format, SHA-256 length, etc.) is
/// intentionally deferred.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IocValue(String);

impl IocValue {
    /// Creates a new `IocValue` from a raw string, trimming surrounding
    /// whitespace.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] when an invariant is violated.
    pub fn new(value: impl Into<String>) -> Result<Self, ValueObjectValidationError> {
        let value = value.into().trim().to_string();

        if value.is_empty() {
            return Err(ValueObjectValidationError::new(
                "ioc value must not be empty".to_string(),
            ));
        }

        if value.chars().count() > MAX_LENGTH {
            return Err(ValueObjectValidationError::new(format!(
                "ioc value must be at most {MAX_LENGTH} characters, got {}",
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

impl std::fmt::Display for IocValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
