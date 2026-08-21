//! Response types for the find-ioc use case.

use std::time::SystemTime;

/// Data entry DTO for an ioc.
pub struct IocEntry {
    pub id: String,
    pub ioc_type: String,
    pub value: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

/// Structured error DTO for ioc operations.
pub struct IocErrorEntry {
    pub message: String,
    pub concept: String,
}

/// Response envelope returned by [`FindIocQueryHandler`].
///
/// [`FindIocQueryHandler`]: super::find_ioc_query_handler::FindIocQueryHandler
pub struct FindIocResponse {
    pub ioc: Option<IocEntry>,
    pub error: Option<IocErrorEntry>,
}
