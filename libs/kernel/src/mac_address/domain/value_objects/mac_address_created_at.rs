//! Value Object for the creation timestamp of a [`MacAddress`].
//!
//! [`MacAddress`]: crate::mac_address::domain::entities::mac_address::MacAddress

use std::time::SystemTime;

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
use shared_valueobject::domain::timestamps::microsecond_timestamp::MicrosecondTimestamp;

/// An immutable Value Object recording when Fennec first registered a
/// [`MacAddress`](crate::mac_address::domain::entities::mac_address::MacAddress),
/// with microsecond precision.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MacAddressCreatedAt(MicrosecondTimestamp);

impl MacAddressCreatedAt {
    /// Creates a new `MacAddressCreatedAt` for the current instant.
    pub fn now() -> Self {
        Self(MicrosecondTimestamp::now())
    }

    /// Creates a `MacAddressCreatedAt` from microseconds since the Unix
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
