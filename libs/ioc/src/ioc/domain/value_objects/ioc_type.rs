//! Value Object for the observable type of an [`Ioc`].
//!
//! [`Ioc`]: crate::ioc::domain::entities::ioc::Ioc

use thiserror::Error;

/// Errors returned when constructing an [`IocType`].
#[derive(Debug, Error, PartialEq, Eq)]
pub enum IocTypeError {
    /// The given string does not match any known ioc type.
    #[error("invalid ioc type: {0}")]
    Invalid(String),
}

/// An immutable Value Object representing the observable kind of an
/// [`Ioc`](crate::ioc::domain::entities::ioc::Ioc).
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum IocType {
    /// IPv4 address.
    Ipv4,
    /// IPv6 address.
    Ipv6,
    /// DNS domain name.
    Domain,
    /// URL.
    Url,
    /// SHA-256 file hash.
    Sha256,
    /// SHA-1 file hash.
    Sha1,
    /// MD5 file hash.
    Md5,
    /// Email address.
    Email,
}

impl IocType {
    /// Parses a raw string into an `IocType`.
    ///
    /// # Errors
    ///
    /// Returns [`IocTypeError::Invalid`] if the value is not a recognised
    /// ioc type.
    pub fn from_str(value: &str) -> Result<Self, IocTypeError> {
        match value {
            "ipv4" => Ok(Self::Ipv4),
            "ipv6" => Ok(Self::Ipv6),
            "domain" => Ok(Self::Domain),
            "url" => Ok(Self::Url),
            "sha256" => Ok(Self::Sha256),
            "sha1" => Ok(Self::Sha1),
            "md5" => Ok(Self::Md5),
            "email" => Ok(Self::Email),
            other => Err(IocTypeError::Invalid(other.to_string())),
        }
    }

    /// Returns the canonical string representation of this ioc type.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ipv4 => "ipv4",
            Self::Ipv6 => "ipv6",
            Self::Domain => "domain",
            Self::Url => "url",
            Self::Sha256 => "sha256",
            Self::Sha1 => "sha1",
            Self::Md5 => "md5",
            Self::Email => "email",
        }
    }
}

impl std::fmt::Display for IocType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
