//! JSON form of one domain name found by the co-resolution pivot.

use serde::Serialize;

use kernel::dns_resolution::application::list_co_resolved_domain_names::list_co_resolved_domain_names_response::CoResolvedDomainNameEntry;

use super::dns_resolution_endpoint_response::DnsResolutionEndpointResponse;

/// JSON form of a domain name co-resolved with the queried one, as listed
/// in a [`CoResolvedDomainNamesResponse`]: the name and the answers both
/// resolved to at overlapping times, sorted by address.
///
/// [`CoResolvedDomainNamesResponse`]: super::co_resolved_domain_names_response::CoResolvedDomainNamesResponse
#[derive(Serialize)]
pub struct CoResolvedDomainNameResponse {
    pub domain_name: DnsResolutionEndpointResponse,
    pub shared_answers: Vec<DnsResolutionEndpointResponse>,
}

impl CoResolvedDomainNameResponse {
    /// Maps a [`CoResolvedDomainNameEntry`] to its JSON form.
    pub fn from_entry(entry: &CoResolvedDomainNameEntry) -> Self {
        Self {
            domain_name: DnsResolutionEndpointResponse::from_entry(&entry.domain_name),
            shared_answers: entry
                .shared_answers
                .iter()
                .map(DnsResolutionEndpointResponse::from_entry)
                .collect(),
        }
    }
}
