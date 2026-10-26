//! Response types for the find-domain-name use case.

use std::time::SystemTime;

/// Data entry DTO for a domain name.
pub struct DomainNameEntry {
    pub id: String,
    pub stix_id: String,
    pub value: String,
    pub created_at: SystemTime,
}

/// Structured error DTO for domain name operations.
pub struct DomainNameErrorEntry {
    pub message: String,
    pub concept: String,
}

/// Response envelope returned by [`FindDomainNameQueryHandler`].
///
/// [`FindDomainNameQueryHandler`]: super::find_domain_name_query_handler::FindDomainNameQueryHandler
pub struct FindDomainNameResponse {
    pub domain_name: Option<DomainNameEntry>,
    pub error: Option<DomainNameErrorEntry>,
}
