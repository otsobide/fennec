//! Response types for the find-mac-address use case.

use std::time::SystemTime;

/// Data entry DTO for a MAC address.
pub struct MacAddressEntry {
    pub id: String,
    pub stix_id: String,
    pub value: String,
    pub created_at: SystemTime,
}

/// Structured error DTO for MAC address operations.
pub struct MacAddressErrorEntry {
    pub message: String,
    pub concept: String,
}

/// Response envelope returned by [`FindMacAddressQueryHandler`].
///
/// [`FindMacAddressQueryHandler`]: super::find_mac_address_query_handler::FindMacAddressQueryHandler
pub struct FindMacAddressResponse {
    pub mac_address: Option<MacAddressEntry>,
    pub error: Option<MacAddressErrorEntry>,
}
