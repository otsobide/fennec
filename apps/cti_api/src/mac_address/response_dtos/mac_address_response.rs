use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use kernel::mac_address::application::find_mac_address::find_mac_address_response::MacAddressEntry;

/// JSON body returned by `GET /mac-addresses/{id}` and
/// `GET /mac-addresses?value=<address>`.
#[derive(Serialize)]
pub struct MacAddressResponse {
    pub id: String,
    pub stix_id: String,
    pub value: String,
    pub created_at: u64,
}

impl MacAddressResponse {
    pub fn from_entry(entry: &MacAddressEntry) -> Self {
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
