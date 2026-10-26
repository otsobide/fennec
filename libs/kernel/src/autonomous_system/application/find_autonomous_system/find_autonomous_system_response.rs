//! Response types for the find-autonomous-system use case.

use std::time::SystemTime;

/// Data entry DTO for an autonomous system.
pub struct AutonomousSystemEntry {
    pub id: String,
    pub stix_id: String,
    pub value: String,
    pub name: Option<String>,
    pub rir: Option<String>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

/// Structured error DTO for autonomous system operations.
pub struct AutonomousSystemErrorEntry {
    pub message: String,
    pub concept: String,
}

/// Response envelope returned by [`FindAutonomousSystemQueryHandler`].
///
/// [`FindAutonomousSystemQueryHandler`]: super::find_autonomous_system_query_handler::FindAutonomousSystemQueryHandler
pub struct FindAutonomousSystemResponse {
    pub autonomous_system: Option<AutonomousSystemEntry>,
    pub error: Option<AutonomousSystemErrorEntry>,
}
