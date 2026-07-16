//! Value Object for the raw observable value of an [`Ioc`].
//!
//! [`Ioc`]: crate::ioc::domain::entities::ioc::Ioc

use thiserror::Error;

/// Errors returned when constructing an [`IocValue`].
#[derive(Debug, Error, PartialEq, Eq)]
pub enum IocValueError {
    /// The value is empty.
    #[error("ioc value is empty")]
    Empty,
}

/// An immutable Value Object wrapping the observable string of an
/// [`Ioc`](crate::ioc::domain::entities::ioc::Ioc).
///
/// Validated at construction: must be non-empty. Per-type validation (IPv4
/// format, SHA-256 length, etc.) is intentionally deferred.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IocValue(String);

impl IocValue {
    /// Creates a new `IocValue` from a raw string.
    ///
    /// # Errors
    ///
    /// Returns [`IocValueError::Empty`] if the value is empty.
    pub fn new(value: impl Into<String>) -> Result<Self, IocValueError> {
        let value = value.into();
        if value.is_empty() {
            return Err(IocValueError::Empty);
        }
        Ok(Self(value))
    }

    /// Returns a reference to the underlying value string.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for IocValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
