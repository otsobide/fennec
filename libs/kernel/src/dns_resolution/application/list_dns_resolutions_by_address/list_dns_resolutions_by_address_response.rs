//! Response for the list-DNS-resolutions-by-address use case.

use crate::dns_resolution::application::find_dns_resolution::find_dns_resolution_response::{
    DnsResolutionEntry, DnsResolutionErrorEntry,
};

/// Response envelope returned by [`ListDnsResolutionsByAddressQueryHandler`].
///
/// `dns_resolutions` is always populated (empty if none is recorded), with
/// at most [`MAX_LISTED`] resolutions; `truncated` is `true` when more are
/// recorded than it lists. `error` is only set on repository failure.
///
/// [`MAX_LISTED`]: super::dns_resolutions_by_address_lister::MAX_LISTED
/// [`ListDnsResolutionsByAddressQueryHandler`]: super::list_dns_resolutions_by_address_query_handler::ListDnsResolutionsByAddressQueryHandler
pub struct ListDnsResolutionsByAddressResponse {
    pub dns_resolutions: Vec<DnsResolutionEntry>,
    pub truncated: bool,
    pub error: Option<DnsResolutionErrorEntry>,
}
