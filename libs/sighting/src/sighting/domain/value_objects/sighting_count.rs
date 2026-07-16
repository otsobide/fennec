//! Value Object for the observation count of a [`Sighting`].
//!
//! [`Sighting`]: crate::sighting::domain::entities::sighting::Sighting

use thiserror::Error;

/// Errors returned when constructing a [`SightingCount`].
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SightingCountError {
    /// The count is zero, which is not a valid observation count.
    #[error("sighting count must be at least 1")]
    Zero,
}

/// An immutable Value Object wrapping the number of times the referenced
/// observable has been reported by the referenced source.
///
/// Validated at construction: must be `>= 1`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SightingCount(u64);

impl SightingCount {
    /// Returns a `SightingCount` initialised to `1`, used when a sighting is
    /// created for the first time.
    pub fn one() -> Self {
        Self(1)
    }

    /// Creates a `SightingCount` from a raw `u64`.
    ///
    /// # Errors
    ///
    /// Returns [`SightingCountError::Zero`] if the value is zero.
    pub fn from_u64(value: u64) -> Result<Self, SightingCountError> {
        if value == 0 {
            return Err(SightingCountError::Zero);
        }
        Ok(Self(value))
    }

    /// Returns the underlying count value.
    pub fn value(&self) -> u64 {
        self.0
    }

    /// Returns a new `SightingCount` equal to `self + 1`.
    pub fn incremented(&self) -> Self {
        Self(self.0 + 1)
    }
}
