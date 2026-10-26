use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use kernel::domain_name::application::find_domain_name::find_domain_name_response::DomainNameEntry;

/// JSON body returned by `GET /domain-names/{id}` and
/// `GET /domain-names?value=<name>`.
#[derive(Serialize)]
pub struct DomainNameResponse {
    pub id: String,
    pub stix_id: String,
    pub value: String,
    pub created_at: u64,
}

impl DomainNameResponse {
    pub fn from_entry(entry: &DomainNameEntry) -> Self {
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
