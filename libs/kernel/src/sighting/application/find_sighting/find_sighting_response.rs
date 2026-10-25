//! Response types for the find-sighting use case.

use std::time::SystemTime;

/// Data entry DTO for a sighting.
pub struct SightingEntry {
    pub id: String,
    pub ioc_id: String,
    pub source_id: String,
    pub first_seen: SystemTime,
    pub last_seen: SystemTime,
    pub count: u64,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

/// Structured error DTO for sighting operations.
///
/// `existing_id` is populated only when `concept == "PairAlreadyExists"`, in
/// which case it carries the id of the existing sighting for the same
/// `(ioc_id, source_id)` pair.
pub struct SightingErrorEntry {
    pub message: String,
    pub concept: String,
    pub existing_id: Option<String>,
}

/// Response envelope returned by [`FindSightingQueryHandler`].
///
/// [`FindSightingQueryHandler`]: super::find_sighting_query_handler::FindSightingQueryHandler
pub struct FindSightingResponse {
    pub sighting: Option<SightingEntry>,
    pub error: Option<SightingErrorEntry>,
}
