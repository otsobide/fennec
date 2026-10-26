//! Response types for the list-co-resolved-domain-names use case.

use crate::dns_resolution::application::find_dns_resolution::dns_resolution_endpoint_entry::DnsResolutionEndpointEntry;
use crate::dns_resolution::application::find_dns_resolution::find_dns_resolution_response::DnsResolutionErrorEntry;

use super::co_resolved_domain_name::CoResolvedDomainName;

/// Data entry DTO for a [`CoResolvedDomainName`]: the name and the answers it
/// shared with the queried name, sorted by address.
pub struct CoResolvedDomainNameEntry {
    pub domain_name: DnsResolutionEndpointEntry,
    pub shared_answers: Vec<DnsResolutionEndpointEntry>,
}

impl CoResolvedDomainNameEntry {
    /// Maps a [`CoResolvedDomainName`] to its entry.
    pub fn from_co_resolved(co_resolved: &CoResolvedDomainName) -> Self {
        Self {
            domain_name: DnsResolutionEndpointEntry::from_subject(co_resolved.domain_name()),
            shared_answers: co_resolved
                .shared_answers()
                .iter()
                .map(DnsResolutionEndpointEntry::from_answer)
                .collect(),
        }
    }
}

/// Response envelope returned by [`ListCoResolvedDomainNamesQueryHandler`].
///
/// On success, `domain_name` is the queried name, `co_resolved` lists the
/// co-resolved names sorted by name (empty if none), and `truncated` says
/// whether a limit cut the result short. On failure, only `error` is set.
///
/// [`ListCoResolvedDomainNamesQueryHandler`]: super::list_co_resolved_domain_names_query_handler::ListCoResolvedDomainNamesQueryHandler
pub struct ListCoResolvedDomainNamesResponse {
    pub domain_name: Option<DnsResolutionEndpointEntry>,
    pub co_resolved: Vec<CoResolvedDomainNameEntry>,
    pub truncated: bool,
    pub error: Option<DnsResolutionErrorEntry>,
}
