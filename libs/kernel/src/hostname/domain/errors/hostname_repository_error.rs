//! Error types for hostname repository operations.

use thiserror::Error;

/// Errors that can occur during [`HostnameRepository`] operations.
///
/// [`HostnameRepository`]: crate::hostname::domain::repositories::hostname_repository::HostnameRepository
#[derive(Debug, Error)]
pub enum HostnameRepositoryError {
    /// The requested hostname was not found.
    #[error("hostname not found")]
    NotFound,

    /// The hostname is already registered.
    #[error("hostname already exists")]
    AlreadyExists,

    /// An unexpected storage error occurred.
    #[error("unexpected error: {0}")]
    Unexpected(String),
}
