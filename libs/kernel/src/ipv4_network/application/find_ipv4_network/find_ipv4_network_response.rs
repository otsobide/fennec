//! Response types for the find-ipv4-network use case.

use std::time::SystemTime;

/// Data entry DTO for an IPv4 network.
pub struct Ipv4NetworkEntry {
    pub id: String,
    pub stix_id: String,
    pub value: String,
    pub created_at: SystemTime,
}

/// Structured error DTO for IPv4 network operations.
pub struct Ipv4NetworkErrorEntry {
    pub message: String,
    pub concept: String,
}

/// Response envelope returned by [`FindIpv4NetworkQueryHandler`].
///
/// [`FindIpv4NetworkQueryHandler`]: super::find_ipv4_network_query_handler::FindIpv4NetworkQueryHandler
pub struct FindIpv4NetworkResponse {
    pub ipv4_network: Option<Ipv4NetworkEntry>,
    pub error: Option<Ipv4NetworkErrorEntry>,
}
