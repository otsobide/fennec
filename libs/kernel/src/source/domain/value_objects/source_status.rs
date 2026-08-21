//! Value Object for the status of a [`Source`].
//!
//! [`Source`]: crate::source::domain::entities::source::Source

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// An immutable Value Object representing the lifecycle status of a
/// [`Source`](crate::source::domain::entities::source::Source).
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SourceStatus {
    /// The source is enabled and operating.
    Active,
    /// The source is disabled.
    Inactive,
}

impl SourceStatus {
    /// Parses a raw string into a `SourceStatus`, ignoring surrounding whitespace
    /// and letter case.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value is not a recognised
    /// status.
    pub fn from_str(value: &str) -> Result<Self, ValueObjectValidationError> {
        match value.trim().to_ascii_lowercase().as_str() {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            other => Err(ValueObjectValidationError::new(format!(
                "invalid source status: {other}"
            ))),
        }
    }

    /// Returns the canonical string representation of this status.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
        }
    }
}

impl std::fmt::Display for SourceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
