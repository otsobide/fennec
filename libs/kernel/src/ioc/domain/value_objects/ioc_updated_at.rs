//! Value Object for the last-update timestamp of an [`Ioc`].
//!
//! [`Ioc`]: crate::ioc::domain::entities::ioc::Ioc

use std::time::SystemTime;

/// An immutable Value Object wrapping a [`SystemTime`] that records when an
/// [`Ioc`](crate::ioc::domain::entities::ioc::Ioc) was last updated.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IocUpdatedAt(SystemTime);

impl IocUpdatedAt {
    /// Creates a new `IocUpdatedAt` with the current system time.
    pub fn now() -> Self {
        Self(SystemTime::now())
    }

    /// Creates an `IocUpdatedAt` from an explicit [`SystemTime`].
    pub fn from_system_time(value: SystemTime) -> Self {
        Self(value)
    }

    /// Returns the underlying [`SystemTime`].
    pub fn value(&self) -> SystemTime {
        self.0
    }
}
