//! Value Object for the registered name of an [`AutonomousSystem`].
//!
//! [`AutonomousSystem`]: crate::autonomous_system::domain::entities::autonomous_system::AutonomousSystem

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// Maximum length of a name, in characters.
pub const MAX_LENGTH: usize = 255;

/// An immutable Value Object holding the name an autonomous system is
/// registered under (`GOOGLE`, `CLOUDFLARENET`), the STIX `name`.
///
/// Surrounding whitespace is dropped and the case is kept, because names
/// are copied from registry records as written. A name must not be empty,
/// must not hold control characters (a line break would let one record
/// spoof another in exports) and has at most 255 characters. The name does
/// not identify the autonomous system: only its number does.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct AutonomousSystemName(String);

impl AutonomousSystemName {
    /// Validates a raw name.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] naming the violated rule.
    pub fn new(value: &str) -> Result<Self, ValueObjectValidationError> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ValueObjectValidationError::new(
                "autonomous system name must not be empty".to_string(),
            ));
        }
        if trimmed.chars().any(char::is_control) {
            return Err(ValueObjectValidationError::new(format!(
                "autonomous system name must not hold control characters, got {trimmed:?}"
            )));
        }
        let length = trimmed.chars().count();
        if length > MAX_LENGTH {
            return Err(ValueObjectValidationError::new(format!(
                "autonomous system name must be at most {MAX_LENGTH} characters, got {length}"
            )));
        }
        Ok(Self(trimmed.to_string()))
    }

    /// Returns the name.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AutonomousSystemName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
