//! Response types for the find-DNS-resolution use case, shared by every use
//! case that returns resolutions.

use std::time::SystemTime;

use crate::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use crate::dns_resolution::domain::entities::dns_resolution_observation::DnsResolutionObservation;
use crate::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;

use super::dns_resolution_endpoint_entry::DnsResolutionEndpointEntry;

/// Data entry DTO for a [`DnsResolution`] with every source's observation.
///
/// `stix_id` and `relationship_type` are how the resolution is exported to
/// STIX 2.1, a `relationship` of type [`DnsResolutionId::RELATIONSHIP_TYPE`]
/// ([`DnsResolutionId::stix_id`]). `first_seen` and `last_seen` are the
/// window that covers all the observations ([`DnsResolution::window`]).
pub struct DnsResolutionEntry {
    pub id: String,
    pub stix_id: String,
    pub relationship_type: String,
    pub subject: DnsResolutionEndpointEntry,
    pub answer: DnsResolutionEndpointEntry,
    pub first_seen: SystemTime,
    pub last_seen: SystemTime,
    pub created_at: SystemTime,
    pub observations: Vec<DnsResolutionObservationEntry>,
}

impl DnsResolutionEntry {
    /// Maps a [`DnsResolution`] to its entry, so every use case returns it
    /// the same way.
    pub fn from_aggregate(dns_resolution: &DnsResolution) -> Self {
        let window = dns_resolution.window();
        Self {
            id: dns_resolution.id().to_string(),
            stix_id: dns_resolution.id().stix_id(),
            relationship_type: DnsResolutionId::RELATIONSHIP_TYPE.to_string(),
            subject: DnsResolutionEndpointEntry::from_subject(dns_resolution.subject()),
            answer: DnsResolutionEndpointEntry::from_answer(dns_resolution.answer()),
            first_seen: window.first_seen(),
            last_seen: window.last_seen(),
            created_at: dns_resolution.created_at().value(),
            observations: dns_resolution
                .observations()
                .iter()
                .map(DnsResolutionObservationEntry::from_observation)
                .collect(),
        }
    }
}

/// Data entry DTO for one source's [`DnsResolutionObservation`].
pub struct DnsResolutionObservationEntry {
    pub source_id: String,
    pub first_seen: SystemTime,
    pub last_seen: SystemTime,
    pub count: u64,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

impl DnsResolutionObservationEntry {
    /// Maps a [`DnsResolutionObservation`] to its entry.
    pub fn from_observation(observation: &DnsResolutionObservation) -> Self {
        Self {
            source_id: observation.source_id().to_string(),
            first_seen: observation.window().first_seen(),
            last_seen: observation.window().last_seen(),
            count: observation.count().value(),
            created_at: observation.created_at().value(),
            updated_at: observation.updated_at().value(),
        }
    }
}

/// Structured error DTO for DNS resolution operations.
pub struct DnsResolutionErrorEntry {
    pub message: String,
    pub concept: String,
}

/// Response envelope returned by [`FindDnsResolutionQueryHandler`].
///
/// [`FindDnsResolutionQueryHandler`]: super::find_dns_resolution_query_handler::FindDnsResolutionQueryHandler
pub struct FindDnsResolutionResponse {
    pub dns_resolution: Option<DnsResolutionEntry>,
    pub error: Option<DnsResolutionErrorEntry>,
}
