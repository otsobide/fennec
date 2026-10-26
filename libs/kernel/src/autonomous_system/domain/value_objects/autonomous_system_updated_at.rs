//! Value Object for the last update timestamp of an [`AutonomousSystem`].
//!
//! [`AutonomousSystem`]: crate::autonomous_system::domain::entities::autonomous_system::AutonomousSystem

use std::time::SystemTime;

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
use shared_valueobject::domain::timestamps::microsecond_timestamp::MicrosecondTimestamp;

use super::autonomous_system_created_at::AutonomousSystemCreatedAt;

/// An immutable Value Object recording when Fennec last changed the description of an
/// [`AutonomousSystem`](crate::autonomous_system::domain::entities::autonomous_system::AutonomousSystem),
/// with microsecond precision.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AutonomousSystemUpdatedAt(MicrosecondTimestamp);

impl AutonomousSystemUpdatedAt {
    /// Creates a new `AutonomousSystemUpdatedAt` for the current instant.
    pub fn now() -> Self {
        Self(MicrosecondTimestamp::now())
    }

    /// Returns the creation instant as the first update instant: an
    /// autonomous system is last changed when it is created.
    pub fn from_created_at(created_at: &AutonomousSystemCreatedAt) -> Self {
        Self(created_at.timestamp())
    }

    /// Creates an `AutonomousSystemUpdatedAt` from microseconds since the Unix
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
