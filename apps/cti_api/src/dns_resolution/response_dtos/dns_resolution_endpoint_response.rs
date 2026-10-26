//! JSON form of an endpoint of a DNS resolution.

use serde::Serialize;

use kernel::dns_resolution::application::find_dns_resolution::dns_resolution_endpoint_entry::DnsResolutionEndpointEntry;

/// JSON form of an endpoint of a DNS resolution, its subject or its answer:
/// the module kind (`domain_name`, `ipv4_address` or `ipv6_address`), the
/// canonical value and the id the endpoint has in its own module.
#[derive(Serialize)]
pub struct DnsResolutionEndpointResponse {
    pub kind: String,
    pub value: String,
    pub id: String,
}

impl DnsResolutionEndpointResponse {
    /// Maps a [`DnsResolutionEndpointEntry`] to its JSON form.
    pub fn from_entry(entry: &DnsResolutionEndpointEntry) -> Self {
        Self {
            kind: entry.kind.clone(),
            value: entry.value.clone(),
            id: entry.id.clone(),
        }
    }
}
