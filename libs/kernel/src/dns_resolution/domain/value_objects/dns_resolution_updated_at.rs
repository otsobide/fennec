//! Value Object for the last update timestamp of a [`DnsResolution`].
//!
//! [`DnsResolution`]: crate::dns_resolution::domain::entities::dns_resolution::DnsResolution

use std::time::SystemTime;

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
use shared_valueobject::domain::timestamps::microsecond_timestamp::MicrosecondTimestamp;

use super::dns_resolution_created_at::DnsResolutionCreatedAt;

/// An immutable Value Object recording when Fennec last changed an observation of a
/// [`DnsResolution`](crate::dns_resolution::domain::entities::dns_resolution::DnsResolution),
/// with microsecond precision.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DnsResolutionUpdatedAt(MicrosecondTimestamp);

impl DnsResolutionUpdatedAt {
    /// Creates a new `DnsResolutionUpdatedAt` for the current instant.
    pub fn now() -> Self {
        Self(MicrosecondTimestamp::now())
    }

    /// Returns the creation instant as the first update instant: an
    /// observation is last changed when it is created.
    pub fn from_created_at(created_at: &DnsResolutionCreatedAt) -> Self {
        Self(created_at.timestamp())
    }

    /// Creates a `DnsResolutionUpdatedAt` from microseconds since the Unix
    /// epoch, as a persistent store returns it.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the instant is after the end
    /// of year 9999.
    pub fn from_unix_micros(micros: u64) -> Result<Self, ValueObjectValidationError> {
        MicrosecondTimestamp::from_unix_micros(micros).map(Self)
    }

    /// Returns the underlying [`SystemTime`].
    pub fn value(&self) -> SystemTime {
        self.0.value()
    }

    /// Returns the underlying [`MicrosecondTimestamp`].
    pub fn timestamp(&self) -> MicrosecondTimestamp {
        self.0
    }

    /// Returns the number of whole microseconds since the Unix epoch.
    pub fn unix_micros(&self) -> u64 {
        self.0.unix_micros()
    }
}
