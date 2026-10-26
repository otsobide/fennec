//! The rules a [`DnsName`] can break.
//!
//! [`DnsName`]: crate::domain::dns::dns_name::DnsName

use std::fmt;

use super::dns_name::{MAX_LABEL_LENGTH, MAX_LENGTH};

/// The first rule a raw name broke while being parsed as a
/// [`DnsName`](crate::domain::dns::dns_name::DnsName).
///
/// [`DnsNameError::rule`] phrases the rule without a subject, so each
/// observable kind can name itself in its own validation message
/// (`"hostname must not have empty labels"`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DnsNameError {
    /// Nothing is left once the trailing dot is dropped.
    Empty,
    /// UTS #46 processing failed: a code point it disallows (`U+FFFD`, a
    /// line separator) or an invalid A-label (`xn--a`). ASCII passes through
    /// it unchanged and is checked by the label rules instead.
    NotInternationalisable,
    /// The canonical name is longer than [`MAX_LENGTH`] characters.
    TooLong,
    /// A leading dot or two dots in a row.
    EmptyLabel,
    /// A label is longer than [`MAX_LABEL_LENGTH`] characters.
    LabelTooLong,
    /// A label holds a character other than a lowercase letter, a digit, a
    /// hyphen or an underscore.
    InvalidLabelCharacter,
    /// A label starts or ends with a hyphen.
    EdgeHyphen,
    /// The last label is all digits, as in a dotted IPv4 address.
    NumericLastLabel,
}

impl DnsNameError {
    /// Returns the broken rule as a predicate, without a subject.
    pub fn rule(&self) -> String {
        match self {
            Self::Empty => "must not be empty".to_string(),
            Self::NotInternationalisable => "is not a valid internationalised name".to_string(),
            Self::TooLong => format!("must be at most {MAX_LENGTH} characters"),
            Self::EmptyLabel => "must not have empty labels".to_string(),
            Self::LabelTooLong => format!("labels must be at most {MAX_LABEL_LENGTH} characters"),
            Self::InvalidLabelCharacter => {
                "labels may only hold letters, digits, hyphens and underscores".to_string()
            }
            Self::EdgeHyphen => "labels must not start or end with a hyphen".to_string(),
            Self::NumericLastLabel => "must not end in a numeric label".to_string(),
        }
    }
}

impl fmt::Display for DnsNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DNS name {}", self.rule())
    }
}

impl std::error::Error for DnsNameError {}
