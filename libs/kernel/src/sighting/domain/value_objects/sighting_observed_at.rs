//! Value Object for the instant a source observed the sighted observable.

use std::time::SystemTime;

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
use shared_valueobject::domain::timestamps::microsecond_timestamp::MicrosecondTimestamp;

/// An immutable Value Object holding a client-supplied observation instant,
/// between the Unix epoch and the end of year 9999 UTC.
///
/// Observation times arrive from callers as Unix seconds. Bounding them here
/// keeps a hostile or broken value from overflowing [`SystemTime`] or
/// falling outside what PostgreSQL stores.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SightingObservedAt(MicrosecondTimestamp);

impl SightingObservedAt {
    /// Builds an observation instant from Unix seconds.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the instant is after the end
    /// of year 9999.
    pub fn from_unix_seconds(seconds: u64) -> Result<Self, ValueObjectValidationError> {
        let micros = seconds.checked_mul(1_000_000).ok_or_else(|| {
            ValueObjectValidationError::new(format!(
                "sighting observed_at must not be after 9999-12-31T23:59:59Z, got {seconds} seconds"
            ))
        })?;

        MicrosecondTimestamp::from_unix_micros(micros)
            .map(Self)
            .map_err(|_| {
                ValueObjectValidationError::new(format!(
                    "sighting observed_at must not be after 9999-12-31T23:59:59Z, got {seconds} seconds"
                ))
            })
    }

    /// Returns the instant.
    pub fn value(&self) -> SystemTime {
        self.0.value()
    }
}
