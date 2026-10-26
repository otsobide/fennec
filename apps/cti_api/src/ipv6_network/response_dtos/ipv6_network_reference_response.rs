use serde::Serialize;

use kernel::ipv6_network::domain::value_objects::ipv6_network_id::Ipv6NetworkId;

/// JSON body returned by `POST /ipv6-networks`, on success and on conflict:
/// the id the network is registered under and its STIX identifier.
#[derive(Serialize)]
pub struct Ipv6NetworkReferenceResponse {
    pub id: String,
    pub stix_id: String,
}

impl Ipv6NetworkReferenceResponse {
    pub fn from_id(id: &Ipv6NetworkId) -> Self {
        Self {
            id: id.to_string(),
            stix_id: id.stix_id(),
        }
    }
}
