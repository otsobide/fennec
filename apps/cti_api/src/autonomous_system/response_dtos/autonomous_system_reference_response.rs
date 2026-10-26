use serde::Serialize;

use kernel::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;

/// JSON body returned by `POST /autonomous-systems`, on success and on conflict:
/// the id the autonomous system is registered under and its STIX identifier.
#[derive(Serialize)]
pub struct AutonomousSystemReferenceResponse {
    pub id: String,
    pub stix_id: String,
}

impl AutonomousSystemReferenceResponse {
    pub fn from_id(id: &AutonomousSystemId) -> Self {
        Self {
            id: id.to_string(),
            stix_id: id.stix_id(),
        }
    }
}
