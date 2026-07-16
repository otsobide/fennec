//! Value Object for the unique identifier of an [`Ioc`].
//!
//! [`Ioc`]: crate::ioc::domain::entities::ioc::Ioc

use uuid::Uuid;

/// An immutable Value Object wrapping a UUID v4 that uniquely identifies an
/// [`Ioc`](crate::ioc::domain::entities::ioc::Ioc).
///
/// The identifier is **externally provided** at construction time.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct IocId(Uuid);

impl IocId {
    /// Creates a new `IocId` from a UUID supplied by the caller.
    pub fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }

    /// Returns a reference to the underlying UUID.
    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl std::fmt::Display for IocId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
