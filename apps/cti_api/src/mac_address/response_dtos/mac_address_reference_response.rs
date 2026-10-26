use serde::Serialize;

use kernel::mac_address::domain::value_objects::mac_address_id::MacAddressId;

/// JSON body returned by `POST /mac-addresses`, on success and on conflict:
/// the id the address is registered under and its STIX identifier.
#[derive(Serialize)]
pub struct MacAddressReferenceResponse {
    pub id: String,
    pub stix_id: String,
}

impl MacAddressReferenceResponse {
    pub fn from_id(id: &MacAddressId) -> Self {
        Self {
            id: id.to_string(),
            stix_id: id.stix_id(),
        }
    }
}
