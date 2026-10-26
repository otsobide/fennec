//! Value Object for the name a [`DnsResolution`] resolves.
//!
//! [`DnsResolution`]: crate::dns_resolution::domain::entities::dns_resolution::DnsResolution

use shared_valueobject::domain::canonical_json::canonical_json_value::CanonicalJsonValue;
use shared_valueobject::domain::dns::dns_name::DnsName;
use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
use shared_valueobject::domain::stix::stix_deterministic_uuid::StixDeterministicUuid;
use uuid::Uuid;

/// The subject of a DNS resolution: the name that was resolved, typed by
/// its observable kind.
///
/// It is parsed with the rules of the `domain_name` module (surrounding
/// ASCII whitespace dropped, a shared [`DnsName`] with at least two labels),
/// and its id is the one that module derives, so a link points at the same
/// aggregate without importing it. Conformance tests in `tests/` keep the
/// two in step.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum DnsResolutionSubject {
    /// A domain name, an observable of the `domain_name` module.
    DomainName(DnsName),
}

impl DnsResolutionSubject {
    /// Parses a raw domain name as the subject.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] naming the violated rule, worded
    /// as the `domain_name` module words it.
    pub fn domain_name(value: &str) -> Result<Self, ValueObjectValidationError> {
        let trimmed = value.trim_matches(|c: char| c.is_ascii_whitespace());
        let name = DnsName::parse(trimmed).map_err(|error| {
            ValueObjectValidationError::new(format!("domain name {}, got {value}", error.rule()))
        })?;
        if name.label_count() < 2 {
            return Err(ValueObjectValidationError::new(format!(
                "domain name must have at least two labels, got {value}"
            )));
        }
        Ok(Self::DomainName(name))
    }

    /// Returns the module kind of the subject (`domain_name`).
    pub fn kind(&self) -> &'static str {
        match self {
            Self::DomainName(_) => "domain_name",
        }
    }

    /// Returns the canonical value of the subject.
    pub fn value(&self) -> &str {
        match self {
            Self::DomainName(name) => name.as_str(),
        }
    }

    /// Returns the id the subject has in its own module: the UUID part of
    /// its STIX identifier.
    pub fn id(&self) -> Uuid {
        let properties =
            CanonicalJsonValue::object([("value", CanonicalJsonValue::string(self.value()))]);
        *StixDeterministicUuid::from_contributing_properties(&properties).value()
    }

    /// Returns the kind-qualified reference that feeds the link id:
    /// `{"id": "<uuid>", "kind": "<module>"}`.
    pub fn reference(&self) -> CanonicalJsonValue {
        CanonicalJsonValue::object([
            ("id", CanonicalJsonValue::string(self.id().to_string())),
            ("kind", CanonicalJsonValue::string(self.kind())),
        ])
    }
}

impl std::fmt::Display for DnsResolutionSubject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}
