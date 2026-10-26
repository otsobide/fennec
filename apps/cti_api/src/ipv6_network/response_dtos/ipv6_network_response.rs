use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use kernel::ipv6_network::application::find_ipv6_network::find_ipv6_network_response::Ipv6NetworkEntry;

/// JSON body returned by `GET /ipv6-networks/{id}` and
/// `GET /ipv6-networks?value=<network>`.
#[derive(Serialize)]
pub struct Ipv6NetworkResponse {
    pub id: String,
    pub stix_id: String,
    pub value: String,
    pub created_at: u64,
}

impl Ipv6NetworkResponse {
    pub fn from_entry(entry: &Ipv6NetworkEntry) -> Self {
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
