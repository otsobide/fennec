//! Value Object for the address a [`DnsResolution`] resolves to.
//!
//! [`DnsResolution`]: crate::dns_resolution::domain::entities::dns_resolution::DnsResolution

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

use shared_valueobject::domain::canonical_json::canonical_json_value::CanonicalJsonValue;
use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
use shared_valueobject::domain::stix::stix_deterministic_uuid::StixDeterministicUuid;
use uuid::Uuid;

/// The answer of a DNS resolution: the address the name resolved to, typed
/// by its observable kind.
///
/// Each variant is parsed with the rules of its observable module
/// (`ipv4_address`: a dotted quad, optionally `/32`; `ipv6_address`: any
/// RFC 4291 form, optionally `/128`, no zone index), and its id is the one
/// that module derives. The kind is always stated by the caller, never
/// inferred from the value.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum DnsResolutionAnswer {
    /// An IPv4 host address, an observable of the `ipv4_address` module.
    Ipv4Address(Ipv4Addr),
    /// An IPv6 host address, an observable of the `ipv6_address` module.
    Ipv6Address(Ipv6Addr),
}

impl DnsResolutionAnswer {
    /// Parses a raw IPv4 host address as the answer.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value is not a single
    /// IPv4 host address.
    pub fn ipv4_address(value: &str) -> Result<Self, ValueObjectValidationError> {
        let address = strip_host_prefix(value, 32, "ipv4 address")?;
        Ipv4Addr::from_str(address)
            .map(Self::Ipv4Address)
            .map_err(|_| {
                ValueObjectValidationError::new(format!(
                    "ipv4 address must be four decimal octets without leading zeros, got {value}"
                ))
            })
    }

    /// Parses a raw IPv6 host address as the answer.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value is not a single
    /// IPv6 host address.
    pub fn ipv6_address(value: &str) -> Result<Self, ValueObjectValidationError> {
        let address = strip_host_prefix(value, 128, "ipv6 address")?;
        if address.contains('%') {
            return Err(ValueObjectValidationError::new(format!(
                "ipv6 address must not carry a zone index, got {value}"
            )));
        }
        Ipv6Addr::from_str(address)
            .map(Self::Ipv6Address)
            .map_err(|_| {
                ValueObjectValidationError::new(format!(
                "ipv6 address must be eight groups of at most four hexadecimal digits, got {value}"
            ))
            })
    }

    /// Wraps an address read back from a store, choosing the variant from
    /// the address family.
    pub fn from_ip(address: IpAddr) -> Self {
        match address {
            IpAddr::V4(address) => Self::Ipv4Address(address),
            IpAddr::V6(address) => Self::Ipv6Address(address),
        }
    }

    /// Returns the module kind of the answer (`ipv4_address` or
    /// `ipv6_address`).
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Ipv4Address(_) => "ipv4_address",
            Self::Ipv6Address(_) => "ipv6_address",
        }
    }

    /// Returns the address.
    pub fn ip(&self) -> IpAddr {
        match self {
            Self::Ipv4Address(address) => IpAddr::V4(*address),
            Self::Ipv6Address(address) => IpAddr::V6(*address),
        }
    }

    /// Returns the id the answer has in its own module: the UUID part of
    /// its STIX identifier.
    pub fn id(&self) -> Uuid {
        let properties =
            CanonicalJsonValue::object([("value", CanonicalJsonValue::string(self.to_string()))]);
        *StixDeterministicUuid::from_contributing_properties(&properties).value()
    }

    /// Returns the kind-qualified reference that feeds the link id:
    /// `{"id": "<uuid>", "kind": "<module>"}`.
    pub fn reference(&self) -> CanonicalJsonValue {
        CanonicalJsonValue::object([
            ("id", CanonicalJsonValue::string(self.id().to_string())),
            ("kind", CanonicalJsonValue::string(self.kind())),
        ])
    }
}

impl std::fmt::Display for DnsResolutionAnswer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ipv4Address(address) => write!(f, "{address}"),
            Self::Ipv6Address(address) => write!(f, "{address}"),
        }
    }
}

/// Trims the value and drops a host prefix (`/32` or `/128`), rejecting
/// any other prefix the way the address modules do.
fn strip_host_prefix<'a>(
    value: &'a str,
    host_length: u8,
    noun: &str,
) -> Result<&'a str, ValueObjectValidationError> {
    let trimmed = value.trim_matches(|c: char| c.is_ascii_whitespace());
    let Some((address, prefix)) = trimmed.split_once('/') else {
        return Ok(trimmed);
    };
    let well_formed = !prefix.is_empty()
        && prefix.bytes().all(|byte| byte.is_ascii_digit())
        && !(prefix.len() > 1 && prefix.starts_with('0'));
    match prefix.parse::<u8>().ok().filter(|length| well_formed && *length <= host_length) {
        Some(length) if length == host_length => Ok(address),
        Some(length) => Err(ValueObjectValidationError::new(format!(
            "{noun} must be a single host, got the /{length} network {trimmed}"
        ))),
        None => Err(ValueObjectValidationError::new(format!(
            "{noun} prefix must be a length from 0 to {host_length} without leading zeros, got /{prefix}"
        ))),
    }
}
