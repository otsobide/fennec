//! Error types for url source repository operations.

use thiserror::Error;

/// Errors that can occur during [`UrlSourceRepository`] operations.
///
/// [`UrlSourceRepository`]: crate::url_source::domain::repositories::url_source_repository::UrlSourceRepository
#[derive(Debug, Error)]
pub enum UrlSourceRepositoryError {
    /// The requested url source was not found.
    #[error("url source not found")]
    NotFound,

    /// A url source with the same id already exists.
    #[error("url source already exists")]
    AlreadyExists,

    /// An unexpected storage error occurred.
    #[error("unexpected error: {0}")]
    Unexpected(String),
}
