//! Value Object for the unique identifier of an [`Ipv6Address`].
//!
//! [`Ipv6Address`]: crate::ipv6_address::domain::entities::ipv6_address::Ipv6Address

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
use shared_valueobject::domain::stix::stix_deterministic_uuid::StixDeterministicUuid;
use uuid::{Uuid, Variant};

use crate::ipv6_address::domain::value_objects::ipv6_address_value::Ipv6AddressValue;

/// An immutable Value Object wrapping the UUID v5 that identifies an
/// IPv6 address.
///
/// The identity of an IPv6 address is its value: the UUID is the UUID part
/// of the address's deterministic STIX 2.1 identifier, derived from the
/// canonical value, so it is never generated and never chosen by a client.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Ipv6AddressId(Uuid);

impl Ipv6AddressId {
    /// The STIX 2.1 type of an IPv6 address.
    pub const STIX_TYPE: &'static str = "ipv6-addr";

    /// Derives the identifier of the given address.
    pub fn from_value(value: &Ipv6AddressValue) -> Self {
        Self(
            *StixDeterministicUuid::from_contributing_properties(&value.contributing_properties())
                .value(),
        )
    }

    /// Parses a raw string into an `Ipv6AddressId`.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value is not a valid
    /// UUID, or if it is not an RFC 4122 version 5 UUID.
    pub fn new(value: &str) -> Result<Self, ValueObjectValidationError> {
        let parsed = Uuid::parse_str(value.trim()).map_err(|_| {
            ValueObjectValidationError::new(format!("IPv6 address id is not a valid UUID: {value}"))
        })?;

        Self::from_uuid(parsed)
    }

    /// Wraps an already-parsed [`Uuid`].
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the UUID is not an RFC 4122
    /// version 5 UUID, the only kind the derivation can produce.
    pub fn from_uuid(value: Uuid) -> Result<Self, ValueObjectValidationError> {
        if value.get_version_num() != 5 || value.get_variant() != Variant::RFC4122 {
            return Err(ValueObjectValidationError::new(format!(
                "IPv6 address id must be a UUID v5 with the RFC 4122 variant, got version {} with the {:?} variant",
                value.get_version_num(),
                value.get_variant()
            )));
        }

        Ok(Self(value))
    }

    /// Returns a reference to the underlying UUID.
    pub fn value(&self) -> &Uuid {
        &self.0
    }

    /// Returns the STIX 2.1 identifier, `ipv6-addr--<uuid>`.
    pub fn stix_id(&self) -> String {
        format!("{}--{}", Self::STIX_TYPE, self.0)
    }
}

impl std::fmt::Display for Ipv6AddressId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
