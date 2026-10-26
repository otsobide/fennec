//! Response for the delete-DNS-resolution use case.

use crate::dns_resolution::application::find_dns_resolution::find_dns_resolution_response::DnsResolutionErrorEntry;

/// Response envelope returned by [`DeleteDnsResolutionCommandHandler`].
///
/// On success, `error` is `None`. On failure, `error` contains the structured error.
///
/// [`DeleteDnsResolutionCommandHandler`]: super::delete_dns_resolution_command_handler::DeleteDnsResolutionCommandHandler
pub struct DeleteDnsResolutionResponse {
    pub error: Option<DnsResolutionErrorEntry>,
}
