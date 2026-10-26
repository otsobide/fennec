//! Value Object for the address of an [`Ipv6Address`].
//!
//! [`Ipv6Address`]: crate::ipv6_address::domain::entities::ipv6_address::Ipv6Address

use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

use shared_valueobject::domain::canonical_json::canonical_json_value::CanonicalJsonValue;
use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// An immutable Value Object holding one IPv6 host address in canonical form.
///
/// Accepts any RFC 4291 text form (full, compressed, mixed with a dotted
/// IPv4 tail, any case), optionally followed by `/128`, and ignores
/// surrounding ASCII whitespace. The canonical form is the RFC 5952 text:
/// lowercase, leading zeros dropped, the longest run of two or more zero
/// groups compressed to `::` (the leftmost on a tie), and IPv4-mapped
/// addresses written as `::ffff:192.0.2.1`. Networks (prefixes `/0` to
/// `/127`), malformed prefixes, zone indices (`fe80::1%eth0`), brackets,
/// IPv4 addresses and anything that is not IPv6 are rejected.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Ipv6AddressValue(Ipv6Addr);

impl Ipv6AddressValue {
    /// Parses and canonicalises a raw IPv6 host address.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value is not a single
    /// IPv6 host address.
    pub fn new(value: &str) -> Result<Self, ValueObjectValidationError> {
        let trimmed = value.trim_matches(|c: char| c.is_ascii_whitespace());

        let address = match trimmed.split_once('/') {
            None => trimmed,
            Some((address, prefix)) => match parse_prefix_length(prefix) {
                Some(128) => address,
                Some(length) => {
                    return Err(ValueObjectValidationError::new(format!(
                        "ipv6 address must be a single host, got the /{length} network {trimmed}"
                    )))
                }
                None => {
                    return Err(ValueObjectValidationError::new(format!(
                        "ipv6 address prefix must be a length from 0 to 128 without leading zeros, got /{prefix}"
                    )))
                }
            },
        };

        if address.contains('%') {
            return Err(ValueObjectValidationError::new(format!(
                "ipv6 address must not carry a zone index, got {value}"
            )));
        }

        Ipv6Addr::from_str(address).map(Self).map_err(|_| {
            ValueObjectValidationError::new(format!(
                "ipv6 address must be eight groups of at most four hexadecimal digits, got {value}"
            ))
        })
    }

    /// Wraps an already-parsed [`Ipv6Addr`].
    pub fn from_ipv6_addr(value: Ipv6Addr) -> Self {
        Self(value)
    }

    /// Returns the underlying address.
    pub fn value(&self) -> &Ipv6Addr {
        &self.0
    }

    /// Returns `true` for `::1`.
    pub fn is_loopback(&self) -> bool {
        self.0.is_loopback()
    }

    /// Returns `true` for the documentation range `2001:db8::/32` of RFC 3849.
    pub fn is_documentation(&self) -> bool {
        let [first, second, ..] = self.0.segments();
        first == 0x2001 && second == 0x0db8
    }

    /// Returns `true` for the unique local range `fc00::/7` of RFC 4193.
    pub fn is_unique_local(&self) -> bool {
        self.0.segments()[0] & 0xfe00 == 0xfc00
    }

    /// Returns `true` for the link-local unicast range `fe80::/10`.
    pub fn is_link_local(&self) -> bool {
        self.0.segments()[0] & 0xffc0 == 0xfe80
    }

    /// Returns the embedded IPv4 address of an IPv4-mapped address
    /// (`::ffff:0:0/96`), which is the same host as that IPv4 address.
    pub fn ipv4_mapped(&self) -> Option<Ipv4Addr> {
        self.0.to_ipv4_mapped()
    }

    /// Returns the STIX ID contributing properties of the address:
    /// `{"value": "<canonical address>"}`.
    pub fn contributing_properties(&self) -> CanonicalJsonValue {
        CanonicalJsonValue::object([("value", CanonicalJsonValue::string(self.to_string()))])
    }
}

impl std::fmt::Display for Ipv6AddressValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Parses a CIDR prefix length: ASCII digits only, no sign, no leading zero,
/// at most 128.
fn parse_prefix_length(prefix: &str) -> Option<u8> {
    let well_formed = !prefix.is_empty()
        && prefix.bytes().all(|byte| byte.is_ascii_digit())
        && !(prefix.len() > 1 && prefix.starts_with('0'));
    if !well_formed {
        return None;
    }

    prefix.parse::<u8>().ok().filter(|length| *length <= 128)
}
