use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use kernel::ipv4_address::application::find_ipv4_address::find_ipv4_address_response::Ipv4AddressEntry;

/// JSON body returned by `GET /ipv4-addresses/{id}` and
/// `GET /ipv4-addresses?value=<address>`.
#[derive(Serialize)]
pub struct Ipv4AddressResponse {
    pub id: String,
    pub stix_id: String,
    pub value: String,
    pub created_at: u64,
}

impl Ipv4AddressResponse {
    pub fn from_entry(entry: &Ipv4AddressEntry) -> Self {
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
