use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use kernel::hostname::application::find_hostname::find_hostname_response::HostnameEntry;

/// JSON body returned by `GET /hostnames/{id}` and
/// `GET /hostnames?value=<hostname>`.
#[derive(Serialize)]
pub struct HostnameResponse {
    pub id: String,
    pub stix_id: String,
    pub value: String,
    pub created_at: u64,
}

impl HostnameResponse {
    pub fn from_entry(entry: &HostnameEntry) -> Self {
        Self {
            id: entry.id.clone(),
            stix_id: entry.stix_id.clone(),
            value: entry.value.clone(),
            created_at: to_unix_secs(entry.created_at),
        }
    }
}

fn to_unix_secs(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
