use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use kernel::autonomous_system::application::find_autonomous_system::find_autonomous_system_response::AutonomousSystemEntry;

/// JSON body returned by `GET /autonomous-systems/{id}` and
/// `GET /autonomous-systems?value=<number>`.
#[derive(Serialize)]
pub struct AutonomousSystemResponse {
    pub id: String,
    pub stix_id: String,
    pub value: String,
    pub name: Option<String>,
    pub rir: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

impl AutonomousSystemResponse {
    pub fn from_entry(entry: &AutonomousSystemEntry) -> Self {
        Self {
            id: entry.id.clone(),
            stix_id: entry.stix_id.clone(),
            value: entry.value.clone(),
            name: entry.name.clone(),
            rir: entry.rir.clone(),
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
