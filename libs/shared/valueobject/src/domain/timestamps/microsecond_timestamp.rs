//! Value Object for a UTC instant stored with microsecond precision.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// Microseconds from the Unix epoch to 9999-12-31T23:59:59.999999Z, the
/// latest instant [`MicrosecondTimestamp`] accepts.
pub const MAX_UNIX_MICROS: u64 = 253_402_300_799_999_999;

/// An immutable Value Object holding an instant between the Unix epoch and
/// the end of year 9999 UTC, truncated to whole microseconds.
///
/// PostgreSQL `timestamptz` stores microseconds while [`SystemTime`] carries
/// nanoseconds, so a value that kept its nanoseconds would compare unequal
/// after a round trip through the database. The range is what both
/// PostgreSQL and the `time` crate represent without loss.
///
/// # Example
///
/// ```rust
/// use std::time::{Duration, UNIX_EPOCH};
/// use shared_valueobject::domain::timestamps::microsecond_timestamp::MicrosecondTimestamp;
///
/// let instant = UNIX_EPOCH + Duration::from_nanos(1_700_000_000_123_456_789);
/// let timestamp = MicrosecondTimestamp::from_system_time(instant).unwrap();
/// assert_eq!(timestamp.unix_micros(), 1_700_000_000_123_456);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct MicrosecondTimestamp(SystemTime);

impl MicrosecondTimestamp {
    /// Creates a timestamp for the current instant.
    ///
    /// # Panics
    ///
    /// Panics if the system clock is set outside the accepted range, which
    /// would make every timestamp the process produces meaningless.
    pub fn now() -> Self {
        Self::from_system_time(SystemTime::now())
            .expect("the system clock must be between 1970 and 9999")
    }

    /// Creates a timestamp from a [`SystemTime`], dropping any sub-microsecond
    /// part.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] when the instant is before the
    /// Unix epoch or after 9999-12-31T23:59:59.999999Z.
    pub fn from_system_time(value: SystemTime) -> Result<Self, ValueObjectValidationError> {
        let since_epoch = value.duration_since(UNIX_EPOCH).map_err(|_| {
            ValueObjectValidationError::new(
                "timestamp must not be before 1970-01-01T00:00:00Z".to_string(),
            )
        })?;

        let micros = u64::try_from(since_epoch.as_micros()).unwrap_or(u64::MAX);
        Self::from_unix_micros(micros)
    }

    /// Creates a timestamp from a number of microseconds since the Unix epoch.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] when the instant is after
    /// 9999-12-31T23:59:59.999999Z.
    pub fn from_unix_micros(micros: u64) -> Result<Self, ValueObjectValidationError> {
        if micros > MAX_UNIX_MICROS {
            return Err(ValueObjectValidationError::new(format!(
                "timestamp must not be after 9999-12-31T23:59:59.999999Z, got {micros} microseconds since the epoch"
            )));
        }

        Ok(Self(UNIX_EPOCH + Duration::from_micros(micros)))
    }

    /// Returns the instant as a [`SystemTime`].
    pub fn value(&self) -> SystemTime {
        self.0
    }

    /// Returns the number of whole microseconds since the Unix epoch.
    pub fn unix_micros(&self) -> u64 {
        let since_epoch = self
            .0
            .duration_since(UNIX_EPOCH)
            .expect("a MicrosecondTimestamp is never before the epoch");
        u64::try_from(since_epoch.as_micros()).expect("a MicrosecondTimestamp fits in u64")
    }

    /// Returns the number of whole seconds since the Unix epoch.
    pub fn unix_seconds(&self) -> u64 {
        self.unix_micros() / 1_000_000
    }
}
