use serde::Serialize;

use kernel::ipv4_address::domain::value_objects::ipv4_address_id::Ipv4AddressId;

/// JSON body returned by `POST /ipv4-addresses`, on success and on conflict:
/// the id the address is registered under and its STIX identifier.
#[derive(Serialize)]
pub struct Ipv4AddressReferenceResponse {
    pub id: String,
    pub stix_id: String,
}

impl Ipv4AddressReferenceResponse {
    pub fn from_id(id: &Ipv4AddressId) -> Self {
        Self {
            id: id.to_string(),
            stix_id: id.stix_id(),
        }
    }
}
