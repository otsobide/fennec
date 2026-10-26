//! Response types for the find-ipv6-network use case.

use std::time::SystemTime;

/// Data entry DTO for an IPv6 network.
pub struct Ipv6NetworkEntry {
    pub id: String,
    pub stix_id: String,
    pub value: String,
    pub created_at: SystemTime,
}

/// Structured error DTO for IPv6 network operations.
pub struct Ipv6NetworkErrorEntry {
    pub message: String,
    pub concept: String,
}

/// Response envelope returned by [`FindIpv6NetworkQueryHandler`].
///
/// [`FindIpv6NetworkQueryHandler`]: super::find_ipv6_network_query_handler::FindIpv6NetworkQueryHandler
pub struct FindIpv6NetworkResponse {
    pub ipv6_network: Option<Ipv6NetworkEntry>,
    pub error: Option<Ipv6NetworkErrorEntry>,
}
