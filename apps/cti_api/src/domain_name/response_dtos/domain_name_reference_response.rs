use serde::Serialize;

use kernel::domain_name::domain::value_objects::domain_name_id::DomainNameId;

/// JSON body returned by `POST /domain-names`, on success and on conflict:
/// the id the name is registered under and its STIX identifier.
#[derive(Serialize)]
pub struct DomainNameReferenceResponse {
    pub id: String,
    pub stix_id: String,
}

impl DomainNameReferenceResponse {
    pub fn from_id(id: &DomainNameId) -> Self {
        Self {
            id: id.to_string(),
            stix_id: id.stix_id(),
        }
    }
}
