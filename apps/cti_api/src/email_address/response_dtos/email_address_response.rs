use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use kernel::email_address::application::find_email_address::find_email_address_response::EmailAddressEntry;

/// JSON body returned by `GET /email-addresses/{id}` and
/// `GET /email-addresses?value=<address>`.
#[derive(Serialize)]
pub struct EmailAddressResponse {
    pub id: String,
    pub stix_id: String,
    pub value: String,
    pub created_at: u64,
}

impl EmailAddressResponse {
    pub fn from_entry(entry: &EmailAddressEntry) -> Self {
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
