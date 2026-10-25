//! Response types for the find-ipv4-address use case.

use std::time::SystemTime;

/// Data entry DTO for an ipv4 address.
pub struct Ipv4AddressEntry {
    pub id: String,
    pub stix_id: String,
    pub value: String,
    pub created_at: SystemTime,
}

/// Structured error DTO for ipv4 address operations.
pub struct Ipv4AddressErrorEntry {
    pub message: String,
    pub concept: String,
}

/// Response envelope returned by [`FindIpv4AddressQueryHandler`].
///
/// [`FindIpv4AddressQueryHandler`]: super::find_ipv4_address_query_handler::FindIpv4AddressQueryHandler
pub struct FindIpv4AddressResponse {
    pub ipv4_address: Option<Ipv4AddressEntry>,
    pub error: Option<Ipv4AddressErrorEntry>,
}
