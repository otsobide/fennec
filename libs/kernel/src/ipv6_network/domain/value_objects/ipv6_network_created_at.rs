//! Value Object for the creation timestamp of an [`Ipv6Network`].
//!
//! [`Ipv6Network`]: crate::ipv6_network::domain::entities::ipv6_network::Ipv6Network

use std::time::SystemTime;

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
use shared_valueobject::domain::timestamps::microsecond_timestamp::MicrosecondTimestamp;

/// An immutable Value Object recording when Fennec first registered an
/// [`Ipv6Network`](crate::ipv6_network::domain::entities::ipv6_network::Ipv6Network),
/// with microsecond precision.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Ipv6NetworkCreatedAt(MicrosecondTimestamp);

impl Ipv6NetworkCreatedAt {
    /// Creates a new `Ipv6NetworkCreatedAt` for the current instant.
    pub fn now() -> Self {
        Self(MicrosecondTimestamp::now())
    }

    /// Creates an `Ipv6NetworkCreatedAt` from microseconds since the Unix
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

    /// Returns the number of whole microseconds since the Unix epoch.
    pub fn unix_micros(&self) -> u64 {
        self.0.unix_micros()
    }
}
