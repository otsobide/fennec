//! Value Object for the address of an [`Ipv4Address`].
//!
//! [`Ipv4Address`]: crate::ipv4_address::domain::entities::ipv4_address::Ipv4Address

use std::net::Ipv4Addr;
use std::str::FromStr;

use shared_valueobject::domain::canonical_json::canonical_json_value::CanonicalJsonValue;
use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// An immutable Value Object holding one IPv4 host address in canonical form.
///
/// Accepts dotted-quad notation, optionally followed by `/32`, and ignores
/// surrounding whitespace. The canonical form is the dotted quad without
/// leading zeros and without a prefix (`198.51.100.7`). Networks (any prefix
/// other than `/32`), octets with leading zeros (ambiguous between decimal
/// and octal) and anything that is not IPv4 are rejected.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Ipv4AddressValue(Ipv4Addr);

impl Ipv4AddressValue {
    /// Parses and canonicalises a raw IPv4 host address.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value is not a single
    /// IPv4 host address.
    pub fn new(value: &str) -> Result<Self, ValueObjectValidationError> {
        let trimmed = value.trim();

        let address = match trimmed.split_once('/') {
            Some((address, "32")) => address,
            Some((_, prefix)) => {
                return Err(ValueObjectValidationError::new(format!(
                    "ipv4 address must be a single host, got the /{prefix} network {trimmed}"
                )))
            }
            None => trimmed,
        };

        Ipv4Addr::from_str(address).map(Self).map_err(|_| {
            ValueObjectValidationError::new(format!(
                "ipv4 address must be four decimal octets without leading zeros, got {value}"
            ))
        })
    }

    /// Wraps an already-parsed [`Ipv4Addr`].
    pub fn from_ipv4_addr(value: Ipv4Addr) -> Self {
        Self(value)
    }

    /// Returns the underlying address.
    pub fn value(&self) -> &Ipv4Addr {
        &self.0
    }

    /// Returns `true` for the private ranges of RFC 1918.
    pub fn is_private(&self) -> bool {
        self.0.is_private()
    }

    /// Returns `true` for `127.0.0.0/8`.
    pub fn is_loopback(&self) -> bool {
        self.0.is_loopback()
    }

    /// Returns `true` for the documentation ranges of RFC 5737.
    pub fn is_documentation(&self) -> bool {
        self.0.is_documentation()
    }

    /// Returns the STIX ID contributing properties of the address:
    /// `{"value": "<canonical address>"}`.
    pub fn contributing_properties(&self) -> CanonicalJsonValue {
        CanonicalJsonValue::object([("value", CanonicalJsonValue::string(self.to_string()))])
    }
}

impl std::fmt::Display for Ipv4AddressValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
