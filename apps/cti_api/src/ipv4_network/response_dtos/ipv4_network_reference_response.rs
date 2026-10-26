use serde::Serialize;

use kernel::ipv4_network::domain::value_objects::ipv4_network_id::Ipv4NetworkId;

/// JSON body returned by `POST /ipv4-networks`, on success and on conflict:
/// the id the network is registered under and its STIX identifier.
#[derive(Serialize)]
pub struct Ipv4NetworkReferenceResponse {
    pub id: String,
    pub stix_id: String,
}

impl Ipv4NetworkReferenceResponse {
    pub fn from_id(id: &Ipv4NetworkId) -> Self {
        Self {
            id: id.to_string(),
            stix_id: id.stix_id(),
        }
    }
}
