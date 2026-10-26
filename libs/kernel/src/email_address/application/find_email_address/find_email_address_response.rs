//! Response types for the find-email-address use case.

use std::time::SystemTime;

/// Data entry DTO for an email address.
pub struct EmailAddressEntry {
    pub id: String,
    pub stix_id: String,
    pub value: String,
    pub created_at: SystemTime,
}

/// Structured error DTO for email address operations.
pub struct EmailAddressErrorEntry {
    pub message: String,
    pub concept: String,
}

/// Response envelope returned by [`FindEmailAddressQueryHandler`].
///
/// [`FindEmailAddressQueryHandler`]: super::find_email_address_query_handler::FindEmailAddressQueryHandler
pub struct FindEmailAddressResponse {
    pub email_address: Option<EmailAddressEntry>,
    pub error: Option<EmailAddressErrorEntry>,
}
