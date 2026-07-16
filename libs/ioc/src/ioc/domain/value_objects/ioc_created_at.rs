//! Value Object for the creation timestamp of an [`Ioc`].
//!
//! [`Ioc`]: crate::ioc::domain::entities::ioc::Ioc

use std::time::SystemTime;

/// An immutable Value Object wrapping a [`SystemTime`] that records when an
/// [`Ioc`](crate::ioc::domain::entities::ioc::Ioc) was created.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IocCreatedAt(SystemTime);

impl IocCreatedAt {
    /// Creates a new `IocCreatedAt` with the current system time.
    pub fn now() -> Self {
        Self(SystemTime::now())
    }

    /// Creates an `IocCreatedAt` from an explicit [`SystemTime`].
    pub fn from_system_time(value: SystemTime) -> Self {
        Self(value)
    }

    /// Returns the underlying [`SystemTime`].
    pub fn value(&self) -> SystemTime {
        self.0
    }
}
