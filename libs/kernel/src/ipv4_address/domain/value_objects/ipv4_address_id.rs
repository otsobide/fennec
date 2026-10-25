//! Value Object for the unique identifier of an [`Ipv4Address`].
//!
//! [`Ipv4Address`]: crate::ipv4_address::domain::entities::ipv4_address::Ipv4Address

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
use shared_valueobject::domain::stix::stix_deterministic_uuid::StixDeterministicUuid;
use uuid::Uuid;

use crate::ipv4_address::domain::value_objects::ipv4_address_value::Ipv4AddressValue;

/// An immutable Value Object wrapping the UUID v5 that identifies an IPv4
/// address.
///
/// The identity of an IPv4 address is its value: the UUID is the UUID part
/// of the address's deterministic STIX 2.1 identifier, derived from the
/// canonical value, so it is never generated and never chosen by a client.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Ipv4AddressId(Uuid);

impl Ipv4AddressId {
    /// The STIX 2.1 type of an IPv4 address.
    pub const STIX_TYPE: &'static str = "ipv4-addr";

    /// Derives the identifier of the given address.
    pub fn from_value(value: &Ipv4AddressValue) -> Self {
        Self(
            *StixDeterministicUuid::from_contributing_properties(&value.contributing_properties())
                .value(),
        )
    }

    /// Parses a raw string into an `Ipv4AddressId`.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value is not a valid
    /// UUID, or if it is not version 5.
    pub fn new(value: &str) -> Result<Self, ValueObjectValidationError> {
        let parsed = Uuid::parse_str(value.trim()).map_err(|_| {
            ValueObjectValidationError::new(format!("ipv4 address id is not a valid UUID: {value}"))
        })?;

        Self::from_uuid(parsed)
    }

    /// Wraps an already-parsed [`Uuid`].
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the UUID is not version 5.
    pub fn from_uuid(value: Uuid) -> Result<Self, ValueObjectValidationError> {
        if value.get_version_num() != 5 {
            return Err(ValueObjectValidationError::new(format!(
                "ipv4 address id must be a UUID v5, got version {}",
                value.get_version_num()
            )));
        }

        Ok(Self(value))
    }

    /// Returns a reference to the underlying UUID.
    pub fn value(&self) -> &Uuid {
        &self.0
    }

    /// Returns the STIX 2.1 identifier, `ipv4-addr--<uuid>`.
    pub fn stix_id(&self) -> String {
        format!("{}--{}", Self::STIX_TYPE, self.0)
    }
}

impl std::fmt::Display for Ipv4AddressId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
