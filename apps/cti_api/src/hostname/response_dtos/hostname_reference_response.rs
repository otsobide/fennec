use serde::Serialize;

use kernel::hostname::domain::value_objects::hostname_id::HostnameId;

/// JSON body returned by `POST /hostnames`, on success and on conflict:
/// the id the hostname is registered under and its STIX identifier.
#[derive(Serialize)]
pub struct HostnameReferenceResponse {
    pub id: String,
    pub stix_id: String,
}

impl HostnameReferenceResponse {
    pub fn from_id(id: &HostnameId) -> Self {
        Self {
            id: id.to_string(),
            stix_id: id.stix_id(),
        }
    }
}
