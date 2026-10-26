//! JSON form of one source's observation of a DNS resolution.

use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use kernel::dns_resolution::application::find_dns_resolution::find_dns_resolution_response::DnsResolutionObservationEntry;

/// JSON form of one source's observation of a DNS resolution, as listed in
/// a [`DnsResolutionResponse`]. Instants are unix seconds.
///
/// [`DnsResolutionResponse`]: super::dns_resolution_response::DnsResolutionResponse
#[derive(Serialize)]
pub struct DnsResolutionObservationResponse {
    pub source_id: String,
    pub first_seen: u64,
    pub last_seen: u64,
    pub count: u64,
    pub created_at: u64,
    pub updated_at: u64,
}

impl DnsResolutionObservationResponse {
    /// Maps a [`DnsResolutionObservationEntry`] to its JSON form.
    pub fn from_entry(entry: &DnsResolutionObservationEntry) -> Self {
        Self {
            source_id: entry.source_id.clone(),
            first_seen: to_unix_secs(entry.first_seen),
            last_seen: to_unix_secs(entry.last_seen),
            count: entry.count,
            created_at: to_unix_secs(entry.created_at),
            updated_at: to_unix_secs(entry.updated_at),
        }
    }
}

fn to_unix_secs(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
