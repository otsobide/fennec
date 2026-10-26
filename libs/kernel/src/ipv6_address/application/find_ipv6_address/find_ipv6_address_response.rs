//! Response types for the find-ipv6-address use case.

use std::time::SystemTime;

/// Data entry DTO for an IPv6 address.
pub struct Ipv6AddressEntry {
    pub id: String,
    pub stix_id: String,
    pub value: String,
    pub created_at: SystemTime,
}

/// Structured error DTO for IPv6 address operations.
pub struct Ipv6AddressErrorEntry {
    pub message: String,
    pub concept: String,
}

/// Response envelope returned by [`FindIpv6AddressQueryHandler`].
///
/// [`FindIpv6AddressQueryHandler`]: super::find_ipv6_address_query_handler::FindIpv6AddressQueryHandler
pub struct FindIpv6AddressResponse {
    pub ipv6_address: Option<Ipv6AddressEntry>,
    pub error: Option<Ipv6AddressErrorEntry>,
}
