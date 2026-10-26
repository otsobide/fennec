//! Value Object for the unique identifier of an [`Ipv6Network`].
//!
//! [`Ipv6Network`]: crate::ipv6_network::domain::entities::ipv6_network::Ipv6Network

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
use shared_valueobject::domain::stix::stix_deterministic_uuid::StixDeterministicUuid;
use uuid::{Uuid, Variant};

use crate::ipv6_network::domain::value_objects::ipv6_network_value::Ipv6NetworkValue;

/// An immutable Value Object wrapping the UUID v5 that identifies an
/// IPv6 network.
///
/// The identity of an IPv6 network is its value: the UUID is the UUID part
/// of the network's deterministic STIX 2.1 identifier, derived from the
/// canonical value, so it is never generated and never chosen by a client.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Ipv6NetworkId(Uuid);

impl Ipv6NetworkId {
    /// The STIX 2.1 type of an IPv6 network.
    pub const STIX_TYPE: &'static str = "ipv6-addr";

    /// Derives the identifier of the given network.
    pub fn from_value(value: &Ipv6NetworkValue) -> Self {
        Self(
            *StixDeterministicUuid::from_contributing_properties(&value.contributing_properties())
                .value(),
        )
    }

    /// Parses a raw string into an `Ipv6NetworkId`.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value is not a valid
    /// UUID, or if it is not an RFC 4122 version 5 UUID.
    pub fn new(value: &str) -> Result<Self, ValueObjectValidationError> {
        let parsed = Uuid::parse_str(value.trim()).map_err(|_| {
            ValueObjectValidationError::new(format!("IPv6 network id is not a valid UUID: {value}"))
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
                "IPv6 network id must be a UUID v5 with the RFC 4122 variant, got version {} with the {:?} variant",
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

impl std::fmt::Display for Ipv6NetworkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
