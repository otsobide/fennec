//! Value Object for the address of a [`MacAddress`].
//!
//! [`MacAddress`]: crate::mac_address::domain::entities::mac_address::MacAddress

use shared_valueobject::domain::canonical_json::canonical_json_value::CanonicalJsonValue;
use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// Number of octets in an EUI-48 address.
pub const OCTETS: usize = 6;

/// An immutable Value Object holding one EUI-48 (MAC-48) address.
///
/// Accepts the four spellings devices and tools print, ignoring surrounding
/// ASCII whitespace and the case of hexadecimal digits:
///
/// - six groups separated by colons (`d2:fb:49:24:37:18`), or by hyphens
///   (`D2-FB-49-24-37-18`), each of one or two digits, because some tools
///   drop leading zeros (`0:1b:63:84:45:e6`);
/// - three groups of four digits separated by dots (`d2fb.4924.3718`);
/// - twelve digits without separators (`d2fb49243718`).
///
/// The canonical form is the one STIX requires: six lowercase two-digit
/// groups separated by colons. Mixed separators, EUI-64 identifiers (eight
/// octets, a kind of their own), defanged spellings and anything else are
/// rejected.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct MacAddressValue([u8; OCTETS]);

impl MacAddressValue {
    /// Parses and canonicalises a raw MAC address.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value is not a single
    /// EUI-48 address in one of the accepted spellings.
    pub fn new(value: &str) -> Result<Self, ValueObjectValidationError> {
        let trimmed = value.trim_matches(|c: char| c.is_ascii_whitespace());

        let groups: Vec<&str> = if trimmed.contains(':') {
            trimmed.split(':').collect()
        } else if trimmed.contains('-') {
            trimmed.split('-').collect()
        } else if trimmed.contains('.') {
            return parse_dotted(trimmed, value);
        } else {
            return parse_bare(trimmed, value);
        };

        if groups.len() == 8 {
            return Err(invalid(
                "mac address must have six octets, not the eight of an EUI-64",
                value,
            ));
        }
        if groups.len() != OCTETS {
            return Err(invalid("mac address must have six octets", value));
        }

        let mut octets = [0u8; OCTETS];
        for (octet, group) in octets.iter_mut().zip(&groups) {
            *octet = parse_hex(group, 1..=2).ok_or_else(|| {
                invalid(
                    "mac address octets must be one or two hexadecimal digits",
                    value,
                )
            })? as u8;
        }
        Ok(Self(octets))
    }

    /// Wraps already-parsed octets.
    pub fn from_octets(octets: [u8; OCTETS]) -> Self {
        Self(octets)
    }

    /// Returns the six octets.
    pub fn octets(&self) -> [u8; OCTETS] {
        self.0
    }

    /// Returns the organisationally unique identifier, the first three
    /// octets, in canonical notation (`d2:fb:49`).
    pub fn oui(&self) -> String {
        format!("{:02x}:{:02x}:{:02x}", self.0[0], self.0[1], self.0[2])
    }

    /// Returns `true` if the group bit is set: the address names a group of
    /// interfaces, not one.
    pub fn is_multicast(&self) -> bool {
        self.0[0] & 0x01 == 0x01
    }

    /// Returns `true` if the address was assigned locally (randomised by a
    /// phone, set on a virtual machine) rather than by the manufacturer.
    pub fn is_locally_administered(&self) -> bool {
        self.0[0] & 0x02 == 0x02
    }

    /// Returns `true` for `ff:ff:ff:ff:ff:ff`.
    pub fn is_broadcast(&self) -> bool {
        self.0 == [0xff; OCTETS]
    }

    /// Returns the STIX ID contributing properties of the address:
    /// `{"value": "<canonical address>"}`.
    pub fn contributing_properties(&self) -> CanonicalJsonValue {
        CanonicalJsonValue::object([("value", CanonicalJsonValue::string(self.to_string()))])
    }
}

impl std::fmt::Display for MacAddressValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [a, b, c, d, e, g] = self.0;
        write!(f, "{a:02x}:{b:02x}:{c:02x}:{d:02x}:{e:02x}:{g:02x}")
    }
}

/// Parses the dotted spelling: three groups of exactly four digits.
fn parse_dotted(trimmed: &str, value: &str) -> Result<MacAddressValue, ValueObjectValidationError> {
    let groups: Vec<&str> = trimmed.split('.').collect();
    if groups.len() != 3 {
        return Err(invalid(
            "mac address in dotted notation must have three groups",
            value,
        ));
    }
    let mut octets = [0u8; OCTETS];
    for (pair, group) in octets.chunks_mut(2).zip(&groups) {
        let word = parse_hex(group, 4..=4).ok_or_else(|| {
            invalid(
                "mac address groups in dotted notation must be four hexadecimal digits",
                value,
            )
        })?;
        pair[0] = (word >> 8) as u8;
        pair[1] = (word & 0xff) as u8;
    }
    Ok(MacAddressValue(octets))
}

/// Parses the spelling without separators: exactly twelve digits.
fn parse_bare(trimmed: &str, value: &str) -> Result<MacAddressValue, ValueObjectValidationError> {
    if trimmed.len() != 2 * OCTETS || !trimmed.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(invalid(
            "mac address without separators must be twelve hexadecimal digits",
            value,
        ));
    }
    let mut octets = [0u8; OCTETS];
    for (octet, index) in octets.iter_mut().zip((0..trimmed.len()).step_by(2)) {
        *octet = u8::from_str_radix(&trimmed[index..index + 2], 16).map_err(|_| {
            invalid(
                "mac address without separators must be twelve hexadecimal digits",
                value,
            )
        })?;
    }
    Ok(MacAddressValue(octets))
}

/// Parses a group of ASCII hexadecimal digits whose length is in `lengths`.
fn parse_hex(group: &str, lengths: std::ops::RangeInclusive<usize>) -> Option<u16> {
    if !lengths.contains(&group.len()) || !group.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return None;
    }
    u16::from_str_radix(group, 16).ok()
}

fn invalid(rule: &str, value: &str) -> ValueObjectValidationError {
    ValueObjectValidationError::new(format!("{rule}, got {value}"))
}
