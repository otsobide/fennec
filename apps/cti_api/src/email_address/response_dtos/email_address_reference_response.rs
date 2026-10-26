use serde::Serialize;

use kernel::email_address::domain::value_objects::email_address_id::EmailAddressId;

/// JSON body returned by `POST /email-addresses`, on success and on conflict:
/// the id the address is registered under and its STIX identifier.
#[derive(Serialize)]
pub struct EmailAddressReferenceResponse {
    pub id: String,
    pub stix_id: String,
}

impl EmailAddressReferenceResponse {
    pub fn from_id(id: &EmailAddressId) -> Self {
        Self {
            id: id.to_string(),
            stix_id: id.stix_id(),
        }
    }
}
