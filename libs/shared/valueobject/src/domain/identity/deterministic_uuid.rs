//! A UUID v5 derived from a canonical JSON name under a given namespace.

use uuid::Uuid;

use crate::domain::canonical_json::canonical_json_value::CanonicalJsonValue;

/// An immutable Value Object holding a UUID v5 over the RFC 8785
/// serialisation of a JSON name, under the namespace the caller chooses.
///
/// [`StixDeterministicUuid`] is the special case of the OASIS namespace for
/// STIX cyber-observables; this type serves every other deterministic id,
/// such as the links between observables, which STIX forbids from using
/// the OASIS namespace.
///
/// [`StixDeterministicUuid`]: crate::domain::stix::stix_deterministic_uuid::StixDeterministicUuid
///
/// # Example
///
/// ```rust
/// use shared_valueobject::domain::canonical_json::canonical_json_value::CanonicalJsonValue;
/// use shared_valueobject::domain::identity::deterministic_uuid::DeterministicUuid;
/// use shared_valueobject::domain::identity::fennec_link_namespace::FENNEC_LINK_NAMESPACE;
///
/// let name = CanonicalJsonValue::object([("kind", CanonicalJsonValue::string("example"))]);
/// let first = DeterministicUuid::derive(&FENNEC_LINK_NAMESPACE, &name);
/// let second = DeterministicUuid::derive(&FENNEC_LINK_NAMESPACE, &name);
/// assert_eq!(first, second);
/// assert_eq!(first.value().get_version_num(), 5);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct DeterministicUuid(Uuid);

impl DeterministicUuid {
    /// Derives the UUID v5 of `name`, serialised with RFC 8785, under
    /// `namespace`.
    pub fn derive(namespace: &Uuid, name: &CanonicalJsonValue) -> Self {
        Self(Uuid::new_v5(
            namespace,
            name.to_canonical_string().as_bytes(),
        ))
    }

    /// Returns the underlying UUID.
    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl std::fmt::Display for DeterministicUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
