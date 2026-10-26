//! JSON form of a list of DNS resolutions.

use serde::Serialize;

use kernel::dns_resolution::application::find_dns_resolution::find_dns_resolution_response::DnsResolutionEntry;

use super::dns_resolution_response::DnsResolutionResponse;

/// JSON body returned by `GET /dns-resolutions?domain_name=<name>`,
/// `?ipv4_address=<address>` and `?ipv6_address=<address>`: the matching
/// resolutions, each as a [`DnsResolutionResponse`], possibly none, and
/// `truncated`, `true` when more are recorded than the list holds.
#[derive(Serialize)]
pub struct DnsResolutionListResponse {
    pub dns_resolutions: Vec<DnsResolutionResponse>,
    pub truncated: bool,
}

impl DnsResolutionListResponse {
    /// Maps the [`DnsResolutionEntry`]s of a list response and its
    /// `truncated` flag to its JSON form, keeping the order of the entries.
    pub fn from_entries(entries: &[DnsResolutionEntry], truncated: bool) -> Self {
        Self {
            dns_resolutions: entries
                .iter()
                .map(DnsResolutionResponse::from_entry)
                .collect(),
            truncated,
        }
    }
}
