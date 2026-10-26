//! Value Object for the CIDR block of an [`Ipv6Network`].
//!
//! [`Ipv6Network`]: crate::ipv6_network::domain::entities::ipv6_network::Ipv6Network

use std::net::Ipv6Addr;
use std::str::FromStr;

use shared_valueobject::domain::canonical_json::canonical_json_value::CanonicalJsonValue;
use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// Longest prefix of a network; a `/128` is a single host.
pub const MAX_PREFIX_LENGTH: u8 = 127;

/// An immutable Value Object holding one IPv6 network in CIDR notation.
///
/// Accepts any RFC 4291 text form of the network address and a prefix
/// length from `/0` to `/127`, ignoring surrounding ASCII whitespace. The
/// address must be the network address itself, with every host bit zero:
/// `2001:db8::7/32` is rejected rather than truncated, as for IPv4 networks.
/// The canonical form is the RFC 5952 text of the address followed by the
/// prefix length (`2001:db8::/32`).
///
/// A missing prefix or a `/128` is a single host, which is an IPv6 address,
/// not a network; both are rejected, as are malformed prefixes, zone
/// indices, brackets and anything that is not IPv6.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Ipv6NetworkValue {
    address: Ipv6Addr,
    prefix_length: u8,
}

impl Ipv6NetworkValue {
    /// Parses and canonicalises a raw IPv6 CIDR block.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value is not an IPv6
    /// network with a prefix from `/0` to `/127` and no host bits set.
    pub fn new(value: &str) -> Result<Self, ValueObjectValidationError> {
        let trimmed = value.trim_matches(|c: char| c.is_ascii_whitespace());

        let Some((address, prefix)) = trimmed.split_once('/') else {
            return Err(invalid(
                "ipv6 network must have a prefix length; without one it is an IPv6 address",
                value,
            ));
        };
        let prefix_length = parse_prefix_length(prefix).ok_or_else(|| {
            invalid(
                "ipv6 network prefix must be a length from 0 to 128 without leading zeros",
                value,
            )
        })?;
        if prefix_length > MAX_PREFIX_LENGTH {
            return Err(invalid(
                "ipv6 network must be shorter than /128; a /128 is an IPv6 address",
                value,
            ));
        }
        if address.contains('%') {
            return Err(invalid("ipv6 network must not carry a zone index", value));
        }

        let address = Ipv6Addr::from_str(address).map_err(|_| {
            invalid(
                "ipv6 network address must be eight groups of at most four hexadecimal digits",
                value,
            )
        })?;
        if u128::from(address) & !mask(prefix_length) != 0 {
            let network = Ipv6Addr::from(u128::from(address) & mask(prefix_length));
            return Err(invalid(
                &format!(
                    "ipv6 network address must have no host bits set (the network is {network}/{prefix_length})"
                ),
                value,
            ));
        }

        Ok(Self {
            address,
            prefix_length,
        })
    }

    /// Returns the network address, the first address of the block.
    pub fn network_address(&self) -> Ipv6Addr {
        self.address
    }

    /// Returns the prefix length, from 0 to 127.
    pub fn prefix_length(&self) -> u8 {
        self.prefix_length
    }

    /// Returns the last address of the block.
    pub fn last_address(&self) -> Ipv6Addr {
        Ipv6Addr::from(u128::from(self.address) | !mask(self.prefix_length))
    }

    /// Returns `true` if `address` falls inside the block.
    pub fn contains(&self, address: Ipv6Addr) -> bool {
        u128::from(address) & mask(self.prefix_length) == u128::from(self.address)
    }

    /// Returns `true` if the whole block lies inside the documentation range
    /// `2001:db8::/32` of RFC 3849.
    pub fn is_documentation(&self) -> bool {
        self.prefix_length >= 32 && self.address.segments()[..2] == [0x2001, 0x0db8]
    }

    /// Returns `true` if the whole block lies inside the unique local range
    /// `fc00::/7` of RFC 4193.
    pub fn is_unique_local(&self) -> bool {
        self.prefix_length >= 7 && self.address.segments()[0] & 0xfe00 == 0xfc00
    }

    /// Returns the STIX ID contributing properties of the network:
    /// `{"value": "<canonical CIDR block>"}`.
    pub fn contributing_properties(&self) -> CanonicalJsonValue {
        CanonicalJsonValue::object([("value", CanonicalJsonValue::string(self.to_string()))])
    }
}

impl std::fmt::Display for Ipv6NetworkValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.address, self.prefix_length)
    }
}

/// The netmask of a prefix length, as an integer.
fn mask(prefix_length: u8) -> u128 {
    if prefix_length == 0 {
        0
    } else {
        u128::MAX << (128 - u32::from(prefix_length))
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

fn invalid(rule: &str, value: &str) -> ValueObjectValidationError {
    ValueObjectValidationError::new(format!("{rule}, got {value}"))
}
