//! Value Object for the CIDR block of an [`Ipv4Network`].
//!
//! [`Ipv4Network`]: crate::ipv4_network::domain::entities::ipv4_network::Ipv4Network

use std::net::Ipv4Addr;
use std::str::FromStr;

use shared_valueobject::domain::canonical_json::canonical_json_value::CanonicalJsonValue;
use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// Longest prefix of a network; a `/32` is a single host.
pub const MAX_PREFIX_LENGTH: u8 = 31;

/// An immutable Value Object holding one IPv4 network in CIDR notation.
///
/// Accepts a dotted-quad network address and a prefix length from `/0` to
/// `/31`, ignoring surrounding ASCII whitespace. The address must be the
/// network address itself, with every host bit zero: `198.51.100.7/24` is
/// rejected rather than truncated, because it usually names the host
/// `198.51.100.7` on that network, and guessing either meaning would merge
/// different observations. The canonical form is the dotted quad without
/// leading zeros followed by the prefix length (`198.51.100.0/24`).
///
/// A missing prefix or a `/32` is a single host, which is an IPv4 address,
/// not a network; both are rejected, as are malformed prefixes, octets with
/// leading zeros and anything that is not IPv4.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Ipv4NetworkValue {
    address: Ipv4Addr,
    prefix_length: u8,
}

impl Ipv4NetworkValue {
    /// Parses and canonicalises a raw IPv4 CIDR block.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value is not an IPv4
    /// network with a prefix from `/0` to `/31` and no host bits set.
    pub fn new(value: &str) -> Result<Self, ValueObjectValidationError> {
        let trimmed = value.trim_matches(|c: char| c.is_ascii_whitespace());

        let Some((address, prefix)) = trimmed.split_once('/') else {
            return Err(invalid(
                "ipv4 network must have a prefix length; without one it is an IPv4 address",
                value,
            ));
        };
        let prefix_length = parse_prefix_length(prefix).ok_or_else(|| {
            invalid(
                "ipv4 network prefix must be a length from 0 to 32 without leading zeros",
                value,
            )
        })?;
        if prefix_length > MAX_PREFIX_LENGTH {
            return Err(invalid(
                "ipv4 network must be shorter than /32; a /32 is an IPv4 address",
                value,
            ));
        }

        let address = Ipv4Addr::from_str(address).map_err(|_| {
            invalid(
                "ipv4 network address must be four decimal octets without leading zeros",
                value,
            )
        })?;
        if u32::from(address) & !mask(prefix_length) != 0 {
            let network = Ipv4Addr::from(u32::from(address) & mask(prefix_length));
            return Err(invalid(
                &format!(
                    "ipv4 network address must have no host bits set (the network is {network}/{prefix_length})"
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
    pub fn network_address(&self) -> Ipv4Addr {
        self.address
    }

    /// Returns the prefix length, from 0 to 31.
    pub fn prefix_length(&self) -> u8 {
        self.prefix_length
    }

    /// Returns the last address of the block.
    pub fn last_address(&self) -> Ipv4Addr {
        Ipv4Addr::from(u32::from(self.address) | !mask(self.prefix_length))
    }

    /// Returns the number of addresses in the block.
    pub fn size(&self) -> u64 {
        1u64 << (32 - u32::from(self.prefix_length))
    }

    /// Returns `true` if `address` falls inside the block.
    pub fn contains(&self, address: Ipv4Addr) -> bool {
        u32::from(address) & mask(self.prefix_length) == u32::from(self.address)
    }

    /// Returns `true` if the whole block lies inside the private ranges of
    /// RFC 1918.
    pub fn is_private(&self) -> bool {
        [
            (Ipv4Addr::new(10, 0, 0, 0), 8),
            (Ipv4Addr::new(172, 16, 0, 0), 12),
            (Ipv4Addr::new(192, 168, 0, 0), 16),
        ]
        .iter()
        .any(|(range, length)| {
            self.prefix_length >= *length
                && u32::from(self.address) & mask(*length) == u32::from(*range)
        })
    }

    /// Returns the STIX ID contributing properties of the network:
    /// `{"value": "<canonical CIDR block>"}`.
    pub fn contributing_properties(&self) -> CanonicalJsonValue {
        CanonicalJsonValue::object([("value", CanonicalJsonValue::string(self.to_string()))])
    }
}

impl std::fmt::Display for Ipv4NetworkValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.address, self.prefix_length)
    }
}

/// The netmask of a prefix length, as an integer.
fn mask(prefix_length: u8) -> u32 {
    if prefix_length == 0 {
        0
    } else {
        u32::MAX << (32 - u32::from(prefix_length))
    }
}

/// Parses a CIDR prefix length: ASCII digits only, no sign, no leading zero,
/// at most 32.
fn parse_prefix_length(prefix: &str) -> Option<u8> {
    let well_formed = !prefix.is_empty()
        && prefix.bytes().all(|byte| byte.is_ascii_digit())
        && !(prefix.len() > 1 && prefix.starts_with('0'));
    if !well_formed {
        return None;
    }

    prefix.parse::<u8>().ok().filter(|length| *length <= 32)
}

fn invalid(rule: &str, value: &str) -> ValueObjectValidationError {
    ValueObjectValidationError::new(format!("{rule}, got {value}"))
}
