//! Value Object for the unique identifier of a [`DnsResolution`].
//!
//! [`DnsResolution`]: crate::dns_resolution::domain::entities::dns_resolution::DnsResolution

use shared_valueobject::domain::canonical_json::canonical_json_value::CanonicalJsonValue;
use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
use shared_valueobject::domain::identity::deterministic_uuid::DeterministicUuid;
use shared_valueobject::domain::identity::fennec_link_namespace::FENNEC_LINK_NAMESPACE;
use uuid::{Uuid, Variant};

use crate::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;
use crate::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;

/// An immutable Value Object identifying a [`DnsResolution`]: a UUID v5 in
/// the link namespace over its kind-qualified endpoints
/// (`docs/data-model/links.md`), so every source that reports the same
/// resolution converges on one aggregate.
///
/// [`DnsResolution`]: crate::dns_resolution::domain::entities::dns_resolution::DnsResolution
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct DnsResolutionId(Uuid);

impl DnsResolutionId {
    /// The link kind that feeds the id.
    pub const KIND: &'static str = "dns_resolution";

    /// Derives the id of the resolution of `subject` to `answer`.
    pub fn from_endpoints(subject: &DnsResolutionSubject, answer: &DnsResolutionAnswer) -> Self {
        let name = CanonicalJsonValue::object([
            ("answer", answer.reference()),
            ("kind", CanonicalJsonValue::string(Self::KIND)),
            ("subject", subject.reference()),
        ]);
        Self(*DeterministicUuid::derive(&FENNEC_LINK_NAMESPACE, &name).value())
    }

    /// Parses an id from a path.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value is not an RFC 4122
    /// UUID v5.
    pub fn new(value: &str) -> Result<Self, ValueObjectValidationError> {
        let parsed = Uuid::parse_str(value.trim()).map_err(|_| {
            ValueObjectValidationError::new(format!(
                "dns resolution id is not a valid UUID: {value}"
            ))
        })?;
        if parsed.get_version_num() != 5 || parsed.get_variant() != Variant::RFC4122 {
            return Err(ValueObjectValidationError::new(format!(
                "dns resolution id must be a UUID v5 with the RFC 4122 variant, got version {} with the {:?} variant",
                parsed.get_version_num(),
                parsed.get_variant()
            )));
        }
        Ok(Self(parsed))
    }

    /// Returns the underlying UUID.
    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl std::fmt::Display for DnsResolutionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
