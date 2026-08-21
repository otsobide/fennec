//! Error types for ioc repository operations.

use thiserror::Error;

/// Errors that can occur during [`IocRepository`] operations.
///
/// [`IocRepository`]: crate::ioc::domain::repositories::ioc_repository::IocRepository
#[derive(Debug, Error)]
pub enum IocRepositoryError {
    /// The requested ioc was not found.
    #[error("ioc not found")]
    NotFound,

    /// An ioc with the same id already exists.
    #[error("ioc already exists")]
    AlreadyExists,

    /// An unexpected storage error occurred.
    #[error("unexpected error: {0}")]
    Unexpected(String),
}
