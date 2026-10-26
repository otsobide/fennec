use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use kernel::ipv4_network::application::find_ipv4_network::find_ipv4_network_response::Ipv4NetworkEntry;

/// JSON body returned by `GET /ipv4-networks/{id}` and
/// `GET /ipv4-networks?value=<network>`.
#[derive(Serialize)]
pub struct Ipv4NetworkResponse {
    pub id: String,
    pub stix_id: String,
    pub value: String,
    pub created_at: u64,
}

impl Ipv4NetworkResponse {
    pub fn from_entry(entry: &Ipv4NetworkEntry) -> Self {
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
