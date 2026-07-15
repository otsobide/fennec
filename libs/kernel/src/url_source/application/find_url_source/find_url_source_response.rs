//! Response types for the find-url-source use case.

use std::time::SystemTime;

/// Data entry DTO for a url source.
pub struct UrlSourceEntry {
    pub id: String,
    pub url: String,
    pub format: String,
    pub polling_interval_seconds: u32,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

/// Structured error DTO for url source operations.
pub struct UrlSourceErrorEntry {
    pub message: String,
    pub concept: String,
}

/// Response envelope returned by [`FindUrlSourceQueryHandler`].
///
/// [`FindUrlSourceQueryHandler`]: super::find_url_source_query_handler::FindUrlSourceQueryHandler
pub struct FindUrlSourceResponse {
    pub url_source: Option<UrlSourceEntry>,
    pub error: Option<UrlSourceErrorEntry>,
}
