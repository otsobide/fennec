//! JSON form of a DNS resolution with its observations.

use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use kernel::dns_resolution::application::find_dns_resolution::find_dns_resolution_response::DnsResolutionEntry;

use super::dns_resolution_endpoint_response::DnsResolutionEndpointResponse;
use super::dns_resolution_observation_response::DnsResolutionObservationResponse;

/// JSON body returned by `GET /dns-resolutions/{id}`, and the form of each
/// resolution in a [`DnsResolutionListResponse`].
///
/// `stix_id` and `relationship_type` are the STIX 2.1 `relationship` the
/// resolution is exported as (`relationship--<id>`, `resolves-to`).
/// `first_seen` and `last_seen` are the window that covers every
/// observation. Instants are unix seconds.
///
/// [`DnsResolutionListResponse`]: super::dns_resolution_list_response::DnsResolutionListResponse
#[derive(Serialize)]
pub struct DnsResolutionResponse {
    pub id: String,
    pub stix_id: String,
    pub relationship_type: String,
    pub subject: DnsResolutionEndpointResponse,
    pub answer: DnsResolutionEndpointResponse,
    pub first_seen: u64,
    pub last_seen: u64,
    pub created_at: u64,
    pub observations: Vec<DnsResolutionObservationResponse>,
}

impl DnsResolutionResponse {
    /// Maps a [`DnsResolutionEntry`] to its JSON form.
    pub fn from_entry(entry: &DnsResolutionEntry) -> Self {
        Self {
            id: entry.id.clone(),
            stix_id: entry.stix_id.clone(),
            relationship_type: entry.relationship_type.clone(),
            subject: DnsResolutionEndpointResponse::from_entry(&entry.subject),
            answer: DnsResolutionEndpointResponse::from_entry(&entry.answer),
            first_seen: to_unix_secs(entry.first_seen),
            last_seen: to_unix_secs(entry.last_seen),
            created_at: to_unix_secs(entry.created_at),
            observations: entry
                .observations
                .iter()
                .map(DnsResolutionObservationResponse::from_entry)
                .collect(),
        }
    }
}

fn to_unix_secs(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
