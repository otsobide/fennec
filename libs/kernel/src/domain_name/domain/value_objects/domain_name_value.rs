//! Value Object for the name of a [`DomainName`].
//!
//! [`DomainName`]: crate::domain_name::domain::entities::domain_name::DomainName

use shared_valueobject::domain::canonical_json::canonical_json_value::CanonicalJsonValue;
use shared_valueobject::domain::dns::dns_name::{self, DnsName};
use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// Maximum length of a domain name, in characters, without its trailing dot.
pub const MAX_LENGTH: usize = dns_name::MAX_LENGTH;

/// Maximum length of one label, in characters.
pub const MAX_LABEL_LENGTH: usize = dns_name::MAX_LABEL_LENGTH;

/// An immutable Value Object holding a DNS domain name in canonical form.
///
/// Canonicalisation follows `docs/data-model/kinds/domain-name.md`: surrounding
/// ASCII whitespace and one trailing dot are dropped, then the name is parsed
/// as a shared [`DnsName`]: UTS #46 nontransitional processing, which
/// lowercases it and turns every internationalised label into its A-label
/// (`faß.de` becomes `xn--fa-hia.de`), and the label rules below.
///
/// The canonical name has at least two labels of 1 to 63 lowercase letters,
/// digits, hyphens or underscores, none starting or ending with a hyphen, at
/// most 253 characters, and a last label that is not numeric, so an IPv4
/// address is never taken for a domain.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct DomainNameValue(DnsName);

impl DomainNameValue {
    /// Parses and canonicalises a raw domain name.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] naming the violated rule.
    pub fn new(value: &str) -> Result<Self, ValueObjectValidationError> {
        let trimmed = value.trim_matches(|c: char| c.is_ascii_whitespace());
        let name = DnsName::parse(trimmed)
            .map_err(|error| invalid(&format!("domain name {}", error.rule()), value))?;
        if name.label_count() < 2 {
            return Err(invalid("domain name must have at least two labels", value));
        }
        Ok(Self(name))
    }

    /// Returns the canonical name.
    pub fn value(&self) -> &str {
        self.0.as_str()
    }

    /// Returns the labels, from the leftmost to the top-level domain.
    pub fn labels(&self) -> impl Iterator<Item = &str> {
        self.0.labels()
    }

    /// Returns the rightmost label.
    pub fn top_level_domain(&self) -> &str {
        self.0.labels().last().unwrap_or(self.0.as_str())
    }

    /// Returns `true` if any label is an A-label (`xn--`), that is, if the
    /// name was internationalised.
    pub fn is_internationalised(&self) -> bool {
        self.0.is_internationalised()
    }

    /// Returns the STIX ID contributing properties of the name:
    /// `{"value": "<canonical name>"}`.
    pub fn contributing_properties(&self) -> CanonicalJsonValue {
        CanonicalJsonValue::object([("value", CanonicalJsonValue::string(self.0.as_str()))])
    }
}

impl std::fmt::Display for DomainNameValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn invalid(rule: &str, value: &str) -> ValueObjectValidationError {
    ValueObjectValidationError::new(format!("{rule}, got {value}"))
}
