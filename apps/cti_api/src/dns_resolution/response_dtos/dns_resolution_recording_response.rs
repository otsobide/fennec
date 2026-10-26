//! JSON form of the outcome of recording a DNS resolution.

use serde::Serialize;

/// JSON body returned by `POST /dns-resolutions`: the id of the resolution
/// the report was about, the STIX 2.1 relationship it is exported as
/// (`relationship--<id>`, `resolves-to`), and what recording it did,
/// `created`, `observed` or `unchanged` ([`DnsResolutionRecording::outcome`]).
///
/// [`DnsResolutionRecording::outcome`]: kernel::dns_resolution::application::record_dns_resolution::dns_resolution_recording::DnsResolutionRecording::outcome
#[derive(Serialize)]
pub struct DnsResolutionRecordingResponse {
    pub id: String,
    pub stix_id: String,
    pub relationship_type: String,
    pub outcome: String,
}
