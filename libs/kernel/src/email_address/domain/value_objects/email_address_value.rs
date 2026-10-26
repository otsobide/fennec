//! Value Object for the address of an [`EmailAddress`].
//!
//! [`EmailAddress`]: crate::email_address::domain::entities::email_address::EmailAddress

use shared_valueobject::domain::canonical_json::canonical_json_value::CanonicalJsonValue;
use shared_valueobject::domain::dns::dns_name::{self, DnsName};
use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
use shared_valueobject::domain::unicode::lowercase_nfc::lowercase_nfc;

/// Maximum length of an address, in octets (the RFC 5321 path limit without
/// its angle brackets).
pub const MAX_LENGTH: usize = 254;

/// Maximum length of the local part, in octets (RFC 5321).
pub const MAX_LOCAL_PART_LENGTH: usize = 64;

/// Maximum length of the domain, in characters, without its trailing dot.
pub const MAX_DOMAIN_LENGTH: usize = dns_name::MAX_LENGTH;

/// Maximum length of one domain label, in characters.
pub const MAX_LABEL_LENGTH: usize = dns_name::MAX_LABEL_LENGTH;

/// Characters other than letters and digits that RFC 5322 allows in an atom.
const ATOM_SPECIALS: &str = "!#$%&'*+-/=?^_`{|}~";

/// An immutable Value Object holding an email address in canonical form.
///
/// Canonicalisation follows `docs/data-model/kinds/email-address.md`:
/// surrounding ASCII whitespace is dropped and the address is split at its
/// only `@`. The local part is lowercased and put in Unicode NFC, so
/// `José@` and its decomposed spelling are one address; the domain is
/// parsed as a shared [`DnsName`] (one trailing dot dropped, UTS #46
/// nontransitional processing, lowercase A-labels).
///
/// The local part is a dot-atom of RFC 5322 atom characters, extended with
/// non-ASCII characters as RFC 6531 allows, of at most 64 octets. Quoted
/// local parts, comments and address literals (`user@[192.0.2.1]`) are
/// rejected. The domain has at least two labels of 1 to 63 lowercase
/// letters, digits, hyphens or underscores, none starting or ending with a
/// hyphen, at most 253 characters, and a last label that is not numeric. The
/// whole address has at most 254 octets.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct EmailAddressValue(String);

impl EmailAddressValue {
    /// Parses and canonicalises a raw email address.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] naming the violated rule.
    pub fn new(value: &str) -> Result<Self, ValueObjectValidationError> {
        let trimmed = value.trim_matches(|c: char| c.is_ascii_whitespace());
        if trimmed.is_empty() {
            return Err(invalid("email address must not be empty", value));
        }

        let mut parts = trimmed.split('@');
        let (Some(raw_local), Some(raw_domain), None) = (parts.next(), parts.next(), parts.next())
        else {
            return Err(invalid("email address must have exactly one @", value));
        };

        let local_part = canonical_local_part(raw_local, value)?;
        let domain = canonical_domain(raw_domain, value)?;

        let address = format!("{local_part}@{domain}");
        if address.len() > MAX_LENGTH {
            return Err(invalid(
                &format!("email address must be at most {MAX_LENGTH} octets"),
                value,
            ));
        }

        Ok(Self(address))
    }

    /// Returns the canonical address.
    pub fn value(&self) -> &str {
        &self.0
    }

    /// Returns the canonical local part, before the `@`.
    pub fn local_part(&self) -> &str {
        self.0.split('@').next().unwrap_or(&self.0)
    }

    /// Returns the canonical domain, after the `@`.
    pub fn domain(&self) -> &str {
        self.0.split('@').nth(1).unwrap_or_default()
    }

    /// Returns `true` if the local part has non-ASCII characters or any
    /// domain label is an A-label (`xn--`).
    pub fn is_internationalised(&self) -> bool {
        !self.local_part().is_ascii()
            || self
                .domain()
                .split('.')
                .any(|label| label.starts_with("xn--"))
    }

    /// Returns the STIX ID contributing properties of the address:
    /// `{"value": "<canonical address>"}`.
    pub fn contributing_properties(&self) -> CanonicalJsonValue {
        CanonicalJsonValue::object([("value", CanonicalJsonValue::string(self.0.clone()))])
    }
}

impl std::fmt::Display for EmailAddressValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn canonical_local_part(raw: &str, value: &str) -> Result<String, ValueObjectValidationError> {
    if raw.is_empty() {
        return Err(invalid("email address local part must not be empty", value));
    }
    if raw.starts_with('"') {
        return Err(invalid(
            "email address local part must not be quoted",
            value,
        ));
    }

    let local = lowercase_nfc(raw);

    if local.len() > MAX_LOCAL_PART_LENGTH {
        return Err(invalid(
            &format!("email address local part must be at most {MAX_LOCAL_PART_LENGTH} octets"),
            value,
        ));
    }
    if local.split('.').any(str::is_empty) {
        return Err(invalid(
            "email address local part must not start or end with a dot or hold two dots in a row",
            value,
        ));
    }
    if !local.chars().all(is_local_part_char) {
        return Err(invalid(
            "email address local part may only hold letters, digits, dots and !#$%&'*+-/=?^_`{|}~",
            value,
        ));
    }
    Ok(local)
}

fn is_local_part_char(c: char) -> bool {
    c.is_ascii_alphanumeric()
        || c == '.'
        || ATOM_SPECIALS.contains(c)
        || (!c.is_ascii() && !c.is_whitespace() && !c.is_control())
}

fn canonical_domain(raw: &str, value: &str) -> Result<String, ValueObjectValidationError> {
    if raw.starts_with('[') {
        return Err(invalid(
            "email address domain must be a domain name, not an address literal",
            value,
        ));
    }
    let name = DnsName::parse(raw)
        .map_err(|error| invalid(&format!("email address domain {}", error.rule()), value))?;
    if name.label_count() < 2 {
        return Err(invalid(
            "email address domain must have at least two labels",
            value,
        ));
    }
    Ok(name.as_str().to_string())
}

fn invalid(rule: &str, value: &str) -> ValueObjectValidationError {
    ValueObjectValidationError::new(format!("{rule}, got {value}"))
}
