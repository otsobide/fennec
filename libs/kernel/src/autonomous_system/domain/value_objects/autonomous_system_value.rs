//! Value Object for the number of an [`AutonomousSystem`].
//!
//! [`AutonomousSystem`]: crate::autonomous_system::domain::entities::autonomous_system::AutonomousSystem

use shared_valueobject::domain::canonical_json::canonical_json_value::CanonicalJsonValue;
use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// An immutable Value Object holding an autonomous system number (ASN), the
/// value that identifies an autonomous system.
///
/// Accepts, ignoring surrounding ASCII whitespace:
///
/// - the plain number in decimal (`15169`);
/// - the number after an `AS` prefix in any case, with no space
///   (`AS15169`, `as15169`), as registries and most feeds write it;
/// - the asdot notation of RFC 5396, two numbers from 0 to 65535 joined by a
///   dot, the first one counting units of 65536 (`1.10` is `65546`), with or
///   without the `AS` prefix.
///
/// Numbers run from 0 to 4294967295 (32-bit ASNs, RFC 6793). The canonical
/// form is the number itself: it displays as plain decimal (`15169`) and
/// feeds the STIX identifier as the JSON integer `number`. Signs, digits
/// other than ASCII, empty numbers and values above the range are rejected.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct AutonomousSystemValue(u32);

impl AutonomousSystemValue {
    /// Parses and canonicalises a raw autonomous system number.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value is not an ASN in
    /// one of the accepted spellings.
    pub fn new(value: &str) -> Result<Self, ValueObjectValidationError> {
        let trimmed = value.trim_matches(|c: char| c.is_ascii_whitespace());
        let digits = match trimmed.get(..2) {
            Some(prefix) if prefix.eq_ignore_ascii_case("as") => &trimmed[2..],
            _ => trimmed,
        };

        let number = match digits.split_once('.') {
            None => parse_decimal(digits).ok_or_else(|| {
                invalid(
                    "autonomous system number must be a decimal number from 0 to 4294967295",
                    value,
                )
            })?,
            Some((high, low)) => {
                let (Some(high), Some(low)) = (parse_half(high), parse_half(low)) else {
                    return Err(invalid(
                        "autonomous system number in asdot notation must be two numbers from 0 to 65535",
                        value,
                    ));
                };
                (high << 16) | low
            }
        };

        Ok(Self(number))
    }

    /// Wraps an already-known number.
    pub fn from_number(number: u32) -> Self {
        Self(number)
    }

    /// Returns the number.
    pub fn number(&self) -> u32 {
        self.0
    }

    /// Returns the number with its `AS` prefix (`AS15169`).
    pub fn label(&self) -> String {
        format!("AS{}", self.0)
    }

    /// Returns `true` for a 32-bit number, one above 65535.
    pub fn is_four_byte(&self) -> bool {
        self.0 > 0xffff
    }

    /// Returns `true` for the private-use ranges of RFC 6996
    /// (64512 to 65534 and 4200000000 to 4294967294).
    pub fn is_private_use(&self) -> bool {
        (64_512..=65_534).contains(&self.0) || (4_200_000_000..=4_294_967_294).contains(&self.0)
    }

    /// Returns `true` for the documentation ranges of RFC 5398
    /// (64496 to 64511 and 65536 to 65551).
    pub fn is_documentation(&self) -> bool {
        (64_496..=64_511).contains(&self.0) || (65_536..=65_551).contains(&self.0)
    }

    /// Returns the STIX ID contributing properties of the autonomous system:
    /// `{"number": <number>}`.
    pub fn contributing_properties(&self) -> CanonicalJsonValue {
        CanonicalJsonValue::object([(
            "number",
            CanonicalJsonValue::integer(i64::from(self.0))
                .expect("every 32-bit number is inside the JSON safe integer range"),
        )])
    }
}

impl std::fmt::Display for AutonomousSystemValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Parses a decimal number of ASCII digits that fits in 32 bits.
fn parse_decimal(digits: &str) -> Option<u32> {
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    digits.parse::<u32>().ok()
}

/// Parses one half of the asdot notation: a decimal number up to 65535.
fn parse_half(digits: &str) -> Option<u32> {
    parse_decimal(digits).filter(|half| *half <= 0xffff)
}

fn invalid(rule: &str, value: &str) -> ValueObjectValidationError {
    ValueObjectValidationError::new(format!("{rule}, got {value}"))
}
