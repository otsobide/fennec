//! JSON form of the result of the co-resolution pivot.

use serde::Serialize;

use kernel::dns_resolution::application::find_dns_resolution::dns_resolution_endpoint_entry::DnsResolutionEndpointEntry;
use kernel::dns_resolution::application::list_co_resolved_domain_names::list_co_resolved_domain_names_response::CoResolvedDomainNameEntry;

use super::co_resolved_domain_name_response::CoResolvedDomainNameResponse;
use super::dns_resolution_endpoint_response::DnsResolutionEndpointResponse;

/// JSON body returned by `GET /dns-resolutions/co-resolved?domain_name=<name>`.
///
/// `domain_name` is the queried name, `co_resolved` the names that shared
/// an address with it at the same time, sorted by name (possibly none), and
/// `truncated` is `true` when a limit cut the pivot short.
#[derive(Serialize)]
pub struct CoResolvedDomainNamesResponse {
    pub domain_name: DnsResolutionEndpointResponse,
    pub co_resolved: Vec<CoResolvedDomainNameResponse>,
    pub truncated: bool,
}

impl CoResolvedDomainNamesResponse {
    /// Maps the parts of a successful [`ListCoResolvedDomainNamesResponse`]
    /// to its JSON form, keeping the order of the co-resolved names.
    ///
    /// [`ListCoResolvedDomainNamesResponse`]: kernel::dns_resolution::application::list_co_resolved_domain_names::list_co_resolved_domain_names_response::ListCoResolvedDomainNamesResponse
    pub fn from_entries(
        domain_name: &DnsResolutionEndpointEntry,
        co_resolved: &[CoResolvedDomainNameEntry],
        truncated: bool,
    ) -> Self {
        Self {
            domain_name: DnsResolutionEndpointResponse::from_entry(domain_name),
            co_resolved: co_resolved
                .iter()
                .map(CoResolvedDomainNameResponse::from_entry)
                .collect(),
            truncated,
        }
    }
}
