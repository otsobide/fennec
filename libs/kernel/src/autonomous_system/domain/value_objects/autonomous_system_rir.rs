//! Value Object for the Regional Internet Registry of an [`AutonomousSystem`].
//!
//! [`AutonomousSystem`]: crate::autonomous_system::domain::entities::autonomous_system::AutonomousSystem

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// The Regional Internet Registry that assigned an autonomous system number,
/// the STIX `rir`.
///
/// STIX leaves `rir` as free text; Fennec only accepts the five registries,
/// so a value can be filtered on and exported the same way every time.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AutonomousSystemRir {
    /// African Network Information Centre.
    Afrinic,
    /// Asia-Pacific Network Information Centre.
    Apnic,
    /// American Registry for Internet Numbers.
    Arin,
    /// Latin America and Caribbean Network Information Centre.
    Lacnic,
    /// Réseaux IP Européens Network Coordination Centre.
    RipeNcc,
}

impl AutonomousSystemRir {
    /// Parses a registry name, ignoring case, surrounding whitespace and the
    /// spaces, hyphens or underscores inside it, so `RIPE NCC`, `ripe-ncc`
    /// and `RIPE` are one registry.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value names none of the
    /// five registries.
    pub fn new(value: &str) -> Result<Self, ValueObjectValidationError> {
        let compact: String = value
            .chars()
            .filter(|c| !matches!(c, ' ' | '-' | '_') && !c.is_whitespace())
            .collect::<String>()
            .to_ascii_uppercase();
        match compact.as_str() {
            "AFRINIC" => Ok(Self::Afrinic),
            "APNIC" => Ok(Self::Apnic),
            "ARIN" => Ok(Self::Arin),
            "LACNIC" => Ok(Self::Lacnic),
            "RIPE" | "RIPENCC" => Ok(Self::RipeNcc),
            _ => Err(ValueObjectValidationError::new(format!(
                "autonomous system rir must be one of AFRINIC, APNIC, ARIN, LACNIC or RIPE NCC, got {value}"
            ))),
        }
    }

    /// Returns the registry's own spelling of its name.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Afrinic => "AFRINIC",
            Self::Apnic => "APNIC",
            Self::Arin => "ARIN",
            Self::Lacnic => "LACNIC",
            Self::RipeNcc => "RIPE NCC",
        }
    }
}

impl std::fmt::Display for AutonomousSystemRir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
