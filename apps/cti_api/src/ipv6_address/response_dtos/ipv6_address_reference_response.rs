use serde::Serialize;

use kernel::ipv6_address::domain::value_objects::ipv6_address_id::Ipv6AddressId;

/// JSON body returned by `POST /ipv6-addresses`, on success and on conflict:
/// the id the address is registered under and its STIX identifier.
#[derive(Serialize)]
pub struct Ipv6AddressReferenceResponse {
    pub id: String,
    pub stix_id: String,
}

impl Ipv6AddressReferenceResponse {
    pub fn from_id(id: &Ipv6AddressId) -> Self {
        Self {
            id: id.to_string(),
            stix_id: id.stix_id(),
        }
    }
}
