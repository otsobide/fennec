//! Error types for domain name repository operations.

use thiserror::Error;

/// Errors that can occur during [`DomainNameRepository`] operations.
///
/// [`DomainNameRepository`]: crate::domain_name::domain::repositories::domain_name_repository::DomainNameRepository
#[derive(Debug, Error)]
pub enum DomainNameRepositoryError {
    /// The requested domain name was not found.
    #[error("domain name not found")]
    NotFound,

    /// The domain name is already registered.
    #[error("domain name already exists")]
    AlreadyExists,

    /// An unexpected storage error occurred.
    #[error("unexpected error: {0}")]
    Unexpected(String),
}
