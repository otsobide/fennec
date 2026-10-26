//! Value Object for the name of a [`Hostname`].
//!
//! [`Hostname`]: crate::hostname::domain::entities::hostname::Hostname

use shared_valueobject::domain::canonical_json::canonical_json_value::CanonicalJsonValue;
use shared_valueobject::domain::dns::dns_name::{self, DnsName};
use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// Maximum length of a hostname, in characters, without its trailing dot.
pub const MAX_LENGTH: usize = dns_name::MAX_LENGTH;

/// Maximum length of one label, in characters.
pub const MAX_LABEL_LENGTH: usize = dns_name::MAX_LABEL_LENGTH;

/// An immutable Value Object holding a hostname in canonical form.
///
/// A hostname names one machine, with or without a domain (`build-agent-7`,
/// `mail01.corp.example`); unlike a domain name it may be a single label.
/// Canonicalisation follows `docs/data-model/kinds/hostname.md`: surrounding
/// ASCII whitespace and one trailing dot are dropped, then the name is parsed
/// as a shared [`DnsName`]: UTS #46 nontransitional processing, which
/// lowercases it and turns every internationalised label into its A-label
/// (`faß.de` becomes `xn--fa-hia.de`), and the label rules below.
///
/// The canonical name has one or more labels of 1 to 63 lowercase letters,
/// digits, hyphens or underscores, none starting or ending with a hyphen, at
/// most 253 characters, and a last label that is not numeric, so an IPv4
/// address is never taken for a hostname.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct HostnameValue(DnsName);

impl HostnameValue {
    /// Parses and canonicalises a raw hostname.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] naming the violated rule.
    pub fn new(value: &str) -> Result<Self, ValueObjectValidationError> {
        let trimmed = value.trim_matches(|c: char| c.is_ascii_whitespace());
        let name = DnsName::parse(trimmed)
            .map_err(|error| invalid(&format!("hostname {}", error.rule()), value))?;
        Ok(Self(name))
    }

    /// Returns the canonical name.
    pub fn value(&self) -> &str {
        self.0.as_str()
    }

    /// Returns the labels, from the machine name to the rightmost label.
    pub fn labels(&self) -> impl Iterator<Item = &str> {
        self.0.labels()
    }

    /// Returns the leftmost label, the name of the machine itself.
    pub fn machine_name(&self) -> &str {
        self.0.labels().next().unwrap_or(self.0.as_str())
    }

    /// Returns `true` if the hostname has no domain part.
    pub fn is_single_label(&self) -> bool {
        self.0.label_count() == 1
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

impl std::fmt::Display for HostnameValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn invalid(rule: &str, value: &str) -> ValueObjectValidationError {
    ValueObjectValidationError::new(format!("{rule}, got {value}"))
}
