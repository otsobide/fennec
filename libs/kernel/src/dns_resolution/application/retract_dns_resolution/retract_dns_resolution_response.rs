//! Response for the retract-DNS-resolution use case.

use crate::dns_resolution::application::find_dns_resolution::find_dns_resolution_response::DnsResolutionErrorEntry;

/// Response envelope returned by [`RetractDnsResolutionCommandHandler`].
///
/// On success, `error` is `None`. On failure, `error` contains the structured error.
///
/// [`RetractDnsResolutionCommandHandler`]: super::retract_dns_resolution_command_handler::RetractDnsResolutionCommandHandler
pub struct RetractDnsResolutionResponse {
    pub error: Option<DnsResolutionErrorEntry>,
}
