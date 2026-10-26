//! Error types for email address repository operations.

use thiserror::Error;

/// Errors that can occur during [`EmailAddressRepository`] operations.
///
/// [`EmailAddressRepository`]: crate::email_address::domain::repositories::email_address_repository::EmailAddressRepository
#[derive(Debug, Error)]
pub enum EmailAddressRepositoryError {
    /// The requested email address was not found.
    #[error("email address not found")]
    NotFound,

    /// The email address is already registered.
    #[error("email address already exists")]
    AlreadyExists,

    /// An unexpected storage error occurred.
    #[error("unexpected error: {0}")]
    Unexpected(String),
}
