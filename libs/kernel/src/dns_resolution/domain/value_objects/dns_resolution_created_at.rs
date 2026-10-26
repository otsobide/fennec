//! Value Object for the creation timestamp of a [`DnsResolution`].
//!
//! [`DnsResolution`]: crate::dns_resolution::domain::entities::dns_resolution::DnsResolution

use std::time::SystemTime;

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
use shared_valueobject::domain::timestamps::microsecond_timestamp::MicrosecondTimestamp;

use super::dns_resolution_updated_at::DnsResolutionUpdatedAt;

/// An immutable Value Object recording when Fennec first registered a
/// [`DnsResolution`](crate::dns_resolution::domain::entities::dns_resolution::DnsResolution),
/// with microsecond precision.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DnsResolutionCreatedAt(MicrosecondTimestamp);

impl DnsResolutionCreatedAt {
    /// Creates a new `DnsResolutionCreatedAt` for the current instant.
    pub fn now() -> Self {
        Self(MicrosecondTimestamp::now())
    }

    /// Returns an update instant as a creation instant, for an observation
    /// added to a resolution that already exists.
    pub fn from_updated_at(updated_at: &DnsResolutionUpdatedAt) -> Self {
        Self(updated_at.timestamp())
    }

    /// Creates a `DnsResolutionCreatedAt` from microseconds since the Unix
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
