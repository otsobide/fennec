//! The UUID part of a deterministic STIX 2.1 cyber-observable identifier.

use uuid::Uuid;

use crate::domain::canonical_json::canonical_json_value::CanonicalJsonValue;
use crate::domain::identity::deterministic_uuid::DeterministicUuid;

/// An immutable Value Object holding the UUID part of a deterministic STIX
/// 2.1 identifier (`<stix-type>--<uuid>`).
///
/// STIX 2.1 section 2.9 derives it as a UUID v5 in the OASIS namespace
/// ([`StixDeterministicUuid::NAMESPACE`]) over the RFC 8785 serialisation of
/// the object's ID contributing properties: the [`DeterministicUuid`] of
/// those properties in that namespace. The STIX type is not part of the
/// hashed name, so callers that need to tell kinds apart must keep the kind
/// next to the UUID.
///
/// # Example
///
/// ```rust
/// use shared_valueobject::domain::canonical_json::canonical_json_value::CanonicalJsonValue;
/// use shared_valueobject::domain::stix::stix_deterministic_uuid::StixDeterministicUuid;
///
/// let uuid = StixDeterministicUuid::from_contributing_properties(&CanonicalJsonValue::object([
///     ("value", CanonicalJsonValue::string("89.19.209.154")),
/// ]));
/// assert_eq!(uuid.to_string(), "abfb438a-8baf-5f54-b19e-b85ee74b3ed7");
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct StixDeterministicUuid(Uuid);

impl StixDeterministicUuid {
    /// The namespace STIX 2.1 defines for deterministic cyber-observable
    /// identifiers: `00abedb4-aa42-466c-9c01-fed23315a9b7`.
    pub const NAMESPACE: Uuid = Uuid::from_u128(0x00abedb4_aa42_466c_9c01_fed23315a9b7);

    /// Derives the UUID from the ID contributing properties of an object,
    /// given as the JSON object STIX hashes.
    pub fn from_contributing_properties(properties: &CanonicalJsonValue) -> Self {
        Self(*DeterministicUuid::derive(&Self::NAMESPACE, properties).value())
    }

    /// Returns the underlying UUID.
    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl std::fmt::Display for StixDeterministicUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
