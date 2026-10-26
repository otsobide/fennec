//! Value Object for how many times a source saw a [`DnsResolution`].
//!
//! [`DnsResolution`]: crate::dns_resolution::domain::entities::dns_resolution::DnsResolution

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// An immutable Value Object holding the cumulative number of times a source
/// saw a resolution, at least 1.
///
/// Sources such as passive DNS report a running total, so counts are merged
/// with [`DnsResolutionCount::max`], not added: sending the same report twice
/// leaves the count as it was.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DnsResolutionCount(u64);

impl DnsResolutionCount {
    /// The count of a source that reported no count: it saw the resolution
    /// once.
    pub fn one() -> Self {
        Self(1)
    }

    /// Validates a reported count.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] for 0, and for counts above
    /// `i64::MAX`, which a store could not hold.
    pub fn from_u64(value: u64) -> Result<Self, ValueObjectValidationError> {
        if value == 0 || value > i64::MAX as u64 {
            return Err(ValueObjectValidationError::new(format!(
                "dns resolution count must be from 1 to {}, got {value}",
                i64::MAX
            )));
        }
        Ok(Self(value))
    }

    /// Returns the larger of the two counts.
    pub fn max(self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }

    /// Returns the count.
    pub fn value(&self) -> u64 {
        self.0
    }
}
