//! Value Object for the unique identifier of a [`DomainName`].
//!
//! [`DomainName`]: crate::domain_name::domain::entities::domain_name::DomainName

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
use shared_valueobject::domain::stix::stix_deterministic_uuid::StixDeterministicUuid;
use uuid::{Uuid, Variant};

use crate::domain_name::domain::value_objects::domain_name_value::DomainNameValue;

/// An immutable Value Object wrapping the UUID v5 that identifies a
/// domain name.
///
/// The identity of a domain name is its value: the UUID is the UUID part
/// of the name's deterministic STIX 2.1 identifier, derived from the
/// canonical value, so it is never generated and never chosen by a client.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct DomainNameId(Uuid);

impl DomainNameId {
    /// The STIX 2.1 type of a domain name.
    pub const STIX_TYPE: &'static str = "domain-name";

    /// Derives the identifier of the given name.
    pub fn from_value(value: &DomainNameValue) -> Self {
        Self(
            *StixDeterministicUuid::from_contributing_properties(&value.contributing_properties())
                .value(),
        )
    }

    /// Parses a raw string into a `DomainNameId`.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value is not a valid
    /// UUID, or if it is not an RFC 4122 version 5 UUID.
    pub fn new(value: &str) -> Result<Self, ValueObjectValidationError> {
        let parsed = Uuid::parse_str(value.trim()).map_err(|_| {
            ValueObjectValidationError::new(format!("domain name id is not a valid UUID: {value}"))
        })?;

        Self::from_uuid(parsed)
    }

    /// Wraps an already-parsed [`Uuid`].
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the UUID is not an RFC 4122
    /// version 5 UUID, the only kind the derivation can produce.
    pub fn from_uuid(value: Uuid) -> Result<Self, ValueObjectValidationError> {
        if value.get_version_num() != 5 || value.get_variant() != Variant::RFC4122 {
            return Err(ValueObjectValidationError::new(format!(
                "domain name id must be a UUID v5 with the RFC 4122 variant, got version {} with the {:?} variant",
                value.get_version_num(),
                value.get_variant()
            )));
        }

        Ok(Self(value))
    }

    /// Returns a reference to the underlying UUID.
    pub fn value(&self) -> &Uuid {
        &self.0
    }

    /// Returns the STIX 2.1 identifier, `domain-name--<uuid>`.
    pub fn stix_id(&self) -> String {
        format!("{}--{}", Self::STIX_TYPE, self.0)
    }
}

impl std::fmt::Display for DomainNameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
