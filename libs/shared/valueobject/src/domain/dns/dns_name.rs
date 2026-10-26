//! A DNS name in canonical form, shared by every kind that holds one.

use super::dns_name_error::DnsNameError;

/// Maximum length of a DNS name, in characters, without its trailing dot.
pub const MAX_LENGTH: usize = 253;

/// Maximum length of one label, in characters.
pub const MAX_LABEL_LENGTH: usize = 63;

/// An immutable Value Object holding a DNS name in canonical form: lowercase
/// A-labels joined by dots, without a trailing dot.
///
/// [`DnsName::parse`] drops one trailing dot, then applies UTS #46
/// nontransitional processing as the `idna` crate implements it for WHATWG
/// URLs (no STD3 rules), which lowercases the name and turns every
/// internationalised label into its A-label (`faß.de` becomes
/// `xn--fa-hia.de`). The result must have one or more labels of 1 to
/// [`MAX_LABEL_LENGTH`] lowercase letters, digits, hyphens or underscores,
/// none starting or ending with a hyphen, at most [`MAX_LENGTH`] characters
/// in all, and a last label that is not numeric, so a dotted IPv4 address is
/// never taken for a name.
///
/// Observable kinds wrap it and add their own rules: a domain name needs at
/// least two labels, a hostname does not. Surrounding whitespace is the
/// kind's concern too, because an email address must not hide a space after
/// its `@`.
///
/// # Example
///
/// ```rust
/// use shared_valueobject::domain::dns::dns_name::DnsName;
///
/// let name = DnsName::parse("Bücher.Example.").expect("valid name");
/// assert_eq!(name.as_str(), "xn--bcher-kva.example");
/// assert_eq!(name.label_count(), 2);
/// ```
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct DnsName(String);

impl DnsName {
    /// Parses and canonicalises a raw DNS name.
    ///
    /// # Errors
    ///
    /// Returns the [`DnsNameError`] for the first rule the name breaks.
    pub fn parse(raw: &str) -> Result<Self, DnsNameError> {
        let without_root = raw.strip_suffix('.').unwrap_or(raw);
        if without_root.is_empty() {
            return Err(DnsNameError::Empty);
        }

        let ascii = idna::domain_to_ascii(without_root)
            .map_err(|_| DnsNameError::NotInternationalisable)?;

        if ascii.chars().count() > MAX_LENGTH {
            return Err(DnsNameError::TooLong);
        }
        for label in ascii.split('.') {
            validate_label(label)?;
        }
        if ascii
            .rsplit('.')
            .next()
            .is_some_and(|last| last.chars().all(|c| c.is_ascii_digit()))
        {
            return Err(DnsNameError::NumericLastLabel);
        }

        Ok(Self(ascii))
    }

    /// Returns the canonical name.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the labels, from the leftmost to the rightmost.
    pub fn labels(&self) -> impl Iterator<Item = &str> {
        self.0.split('.')
    }

    /// Returns the number of labels.
    pub fn label_count(&self) -> usize {
        self.labels().count()
    }

    /// Returns `true` if any label is an A-label (`xn--`), that is, if the
    /// name was internationalised.
    pub fn is_internationalised(&self) -> bool {
        self.labels().any(|label| label.starts_with("xn--"))
    }
}

impl std::fmt::Display for DnsName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn validate_label(label: &str) -> Result<(), DnsNameError> {
    if label.is_empty() {
        return Err(DnsNameError::EmptyLabel);
    }
    if label.chars().count() > MAX_LABEL_LENGTH {
        return Err(DnsNameError::LabelTooLong);
    }
    if !label
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
    {
        return Err(DnsNameError::InvalidLabelCharacter);
    }
    if label.starts_with('-') || label.ends_with('-') {
        return Err(DnsNameError::EdgeHyphen);
    }
    Ok(())
}
