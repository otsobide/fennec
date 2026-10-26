//! Value Object for the time window in which a source saw a [`DnsResolution`].
//!
//! [`DnsResolution`]: crate::dns_resolution::domain::entities::dns_resolution::DnsResolution

use std::time::SystemTime;

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
use shared_valueobject::domain::timestamps::microsecond_timestamp::MicrosecondTimestamp;

/// An immutable Value Object holding when a source first and last saw a
/// resolution, with microsecond precision and `first_seen <= last_seen`.
///
/// Instants come from the client and are bounded to 1970..9999 like every
/// client-supplied instant. Merging two windows takes the earliest first
/// sighting and the latest last one, so merging is idempotent and does not
/// depend on the order reports arrive in.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct DnsResolutionWindow {
    first_seen: MicrosecondTimestamp,
    last_seen: MicrosecondTimestamp,
}

impl DnsResolutionWindow {
    /// Builds a window from unix seconds, as the HTTP API receives them; a
    /// missing `last_seen` is the same instant as `first_seen`.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if an instant is after year 9999
    /// or `last_seen` is before `first_seen`.
    pub fn from_unix_seconds(
        first_seen: u64,
        last_seen: Option<u64>,
    ) -> Result<Self, ValueObjectValidationError> {
        let first = seconds(first_seen, "first_seen")?;
        let last = last_seen
            .map(|last_seen| seconds(last_seen, "last_seen"))
            .transpose()?
            .unwrap_or(first);
        Self::from_timestamps(first, last)
    }

    /// Builds a window from microseconds since the Unix epoch, as a store
    /// returns them.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if an instant is after year 9999
    /// or `last_seen` is before `first_seen`.
    pub fn from_unix_micros(
        first_seen: u64,
        last_seen: u64,
    ) -> Result<Self, ValueObjectValidationError> {
        Self::from_timestamps(
            MicrosecondTimestamp::from_unix_micros(first_seen)?,
            MicrosecondTimestamp::from_unix_micros(last_seen)?,
        )
    }

    fn from_timestamps(
        first_seen: MicrosecondTimestamp,
        last_seen: MicrosecondTimestamp,
    ) -> Result<Self, ValueObjectValidationError> {
        if last_seen < first_seen {
            return Err(ValueObjectValidationError::new(format!(
                "dns resolution last_seen ({}) must not be before first_seen ({})",
                last_seen.unix_seconds(),
                first_seen.unix_seconds()
            )));
        }
        Ok(Self {
            first_seen,
            last_seen,
        })
    }

    /// Returns the window that covers both windows.
    pub fn merged(&self, other: &Self) -> Self {
        Self {
            first_seen: self.first_seen.min(other.first_seen),
            last_seen: self.last_seen.max(other.last_seen),
        }
    }

    /// Returns `true` if the two windows share at least one instant.
    pub fn overlaps(&self, other: &Self) -> bool {
        self.first_seen <= other.last_seen && other.first_seen <= self.last_seen
    }

    /// Returns when the source first saw the resolution.
    pub fn first_seen(&self) -> SystemTime {
        self.first_seen.value()
    }

    /// Returns when the source last saw the resolution.
    pub fn last_seen(&self) -> SystemTime {
        self.last_seen.value()
    }

    /// Returns `first_seen` in microseconds since the Unix epoch.
    pub fn first_seen_micros(&self) -> u64 {
        self.first_seen.unix_micros()
    }

    /// Returns `last_seen` in microseconds since the Unix epoch.
    pub fn last_seen_micros(&self) -> u64 {
        self.last_seen.unix_micros()
    }
}

fn seconds(value: u64, field: &str) -> Result<MicrosecondTimestamp, ValueObjectValidationError> {
    value
        .checked_mul(1_000_000)
        .ok_or(())
        .and_then(|micros| MicrosecondTimestamp::from_unix_micros(micros).map_err(|_| ()))
        .map_err(|_| {
            ValueObjectValidationError::new(format!(
                "dns resolution {field} must be a unix time in seconds before year 10000, got {value}"
            ))
        })
}
