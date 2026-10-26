//! Value Object for the name of a [`Hostname`].
//!
//! [`Hostname`]: crate::hostname::domain::entities::hostname::Hostname

use shared_valueobject::domain::canonical_json::canonical_json_value::CanonicalJsonValue;
use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// Maximum length of a hostname, in characters, without its trailing dot.
pub const MAX_LENGTH: usize = 253;

/// Maximum length of one label, in characters.
pub const MAX_LABEL_LENGTH: usize = 63;

/// An immutable Value Object holding a hostname in canonical form.
///
/// A hostname names one machine, with or without a domain (`build-agent-7`,
/// `mail01.corp.example`); unlike a domain name it may be a single label.
/// Canonicalisation follows `docs/data-model/kinds/hostname.md`: surrounding
/// ASCII whitespace and one trailing dot are dropped, then the name goes
/// through UTS #46 nontransitional processing as the `idna` crate implements
/// it for WHATWG URLs, which lowercases it and turns every internationalised
/// label into its A-label (`faß.de` becomes `xn--fa-hia.de`).
///
/// The canonical name has one or more labels of 1 to 63 lowercase letters,
/// digits, hyphens or underscores, none starting or ending with a hyphen, at
/// most 253 characters, and a last label that is not numeric, so a
/// hostname is never taken for a hostname.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct HostnameValue(String);

impl HostnameValue {
    /// Parses and canonicalises a raw hostname.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] naming the violated rule.
    pub fn new(value: &str) -> Result<Self, ValueObjectValidationError> {
        let trimmed = value.trim_matches(|c: char| c.is_ascii_whitespace());
        let without_root = trimmed.strip_suffix('.').unwrap_or(trimmed);
        if without_root.is_empty() {
            return Err(invalid("hostname must not be empty", value));
        }

        let ascii = idna::domain_to_ascii(without_root)
            .map_err(|_| invalid("hostname is not a valid internationalised name", value))?;

        if ascii.chars().count() > MAX_LENGTH {
            return Err(invalid(
                &format!("hostname must be at most {MAX_LENGTH} characters"),
                value,
            ));
        }

        let labels: Vec<&str> = ascii.split('.').collect();
        for label in &labels {
            validate_label(label, value)?;
        }
        if labels
            .last()
            .is_some_and(|tld| tld.chars().all(|c| c.is_ascii_digit()))
        {
            return Err(invalid("hostname must not end in a numeric label", value));
        }

        Ok(Self(ascii))
    }

    /// Returns the canonical name.
    pub fn value(&self) -> &str {
        &self.0
    }

    /// Returns the labels, from the machine name to the rightmost label.
    pub fn labels(&self) -> impl Iterator<Item = &str> {
        self.0.split('.')
    }

    /// Returns the leftmost label, the name of the machine itself.
    pub fn machine_name(&self) -> &str {
        self.0.split('.').next().unwrap_or(&self.0)
    }

    /// Returns `true` if the hostname has no domain part.
    pub fn is_single_label(&self) -> bool {
        !self.0.contains('.')
    }

    /// Returns `true` if any label is an A-label (`xn--`), that is, if the
    /// name was internationalised.
    pub fn is_internationalised(&self) -> bool {
        self.labels().any(|label| label.starts_with("xn--"))
    }

    /// Returns the STIX ID contributing properties of the name:
    /// `{"value": "<canonical name>"}`.
    pub fn contributing_properties(&self) -> CanonicalJsonValue {
        CanonicalJsonValue::object([("value", CanonicalJsonValue::string(self.0.clone()))])
    }
}

impl std::fmt::Display for HostnameValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn validate_label(label: &str, value: &str) -> Result<(), ValueObjectValidationError> {
    if label.is_empty() {
        return Err(invalid("hostname must not have empty labels", value));
    }
    if label.chars().count() > MAX_LABEL_LENGTH {
        return Err(invalid(
            &format!("hostname labels must be at most {MAX_LABEL_LENGTH} characters"),
            value,
        ));
    }
    if !label
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
    {
        return Err(invalid(
            "hostname labels may only hold letters, digits, hyphens and underscores",
            value,
        ));
    }
    if label.starts_with('-') || label.ends_with('-') {
        return Err(invalid(
            "hostname labels must not start or end with a hyphen",
            value,
        ));
    }
    Ok(())
}

fn invalid(rule: &str, value: &str) -> ValueObjectValidationError {
    ValueObjectValidationError::new(format!("{rule}, got {value}"))
}
