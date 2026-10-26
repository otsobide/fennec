//! Response for the record-DNS-resolution use case.

use crate::dns_resolution::application::find_dns_resolution::find_dns_resolution_response::DnsResolutionErrorEntry;

/// Response envelope returned by [`RecordDnsResolutionCommandHandler`].
///
/// On success, `id` is the id of the resolution, `stix_id` and
/// `relationship_type` are the STIX 2.1 relationship it is exported as
/// ([`DnsResolutionId::stix_id`], [`DnsResolutionId::RELATIONSHIP_TYPE`]),
/// `outcome` is `created`, `observed` or `unchanged`
/// ([`DnsResolutionRecording::outcome`]), and `error` is `None`. On failure,
/// only `error` is set.
///
/// [`RecordDnsResolutionCommandHandler`]: super::record_dns_resolution_command_handler::RecordDnsResolutionCommandHandler
/// [`DnsResolutionRecording::outcome`]: super::dns_resolution_recording::DnsResolutionRecording::outcome
/// [`DnsResolutionId::stix_id`]: crate::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId::stix_id
/// [`DnsResolutionId::RELATIONSHIP_TYPE`]: crate::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId::RELATIONSHIP_TYPE
pub struct RecordDnsResolutionResponse {
    pub id: Option<String>,
    pub stix_id: Option<String>,
    pub relationship_type: Option<String>,
    pub outcome: Option<String>,
    pub error: Option<DnsResolutionErrorEntry>,
}
