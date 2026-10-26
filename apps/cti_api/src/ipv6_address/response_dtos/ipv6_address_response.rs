use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use kernel::ipv6_address::application::find_ipv6_address::find_ipv6_address_response::Ipv6AddressEntry;

/// JSON body returned by `GET /ipv6-addresses/{id}` and
/// `GET /ipv6-addresses?value=<address>`.
#[derive(Serialize)]
pub struct Ipv6AddressResponse {
    pub id: String,
    pub stix_id: String,
    pub value: String,
    pub created_at: u64,
}

impl Ipv6AddressResponse {
    pub fn from_entry(entry: &Ipv6AddressEntry) -> Self {
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
