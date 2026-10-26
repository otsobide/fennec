//! Response types for the find-hostname use case.

use std::time::SystemTime;

/// Data entry DTO for a hostname.
pub struct HostnameEntry {
    pub id: String,
    pub stix_id: String,
    pub value: String,
    pub created_at: SystemTime,
}

/// Structured error DTO for hostname operations.
pub struct HostnameErrorEntry {
    pub message: String,
    pub concept: String,
}

/// Response envelope returned by [`FindHostnameQueryHandler`].
///
/// [`FindHostnameQueryHandler`]: super::find_hostname_query_handler::FindHostnameQueryHandler
pub struct FindHostnameResponse {
    pub hostname: Option<HostnameEntry>,
    pub error: Option<HostnameErrorEntry>,
}
