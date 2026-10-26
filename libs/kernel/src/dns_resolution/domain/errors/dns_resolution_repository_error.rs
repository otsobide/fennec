//! Error type for [`DnsResolutionRepository`] operations.
//!
//! [`DnsResolutionRepository`]: crate::dns_resolution::domain::repositories::dns_resolution_repository::DnsResolutionRepository

use thiserror::Error;

/// Errors that a [`DnsResolutionRepository`] implementation may return.
///
/// [`DnsResolutionRepository`]: crate::dns_resolution::domain::repositories::dns_resolution_repository::DnsResolutionRepository
#[derive(Debug, Error, PartialEq)]
pub enum DnsResolutionRepositoryError {
    /// No resolution, or no observation of the given source, exists.
    #[error("DNS resolution not found")]
    NotFound,
    /// The resolution is already recorded.
    #[error("DNS resolution already exists")]
    AlreadyExists,
    /// Any other storage failure.
    #[error("Unexpected repository error: {0}")]
    Unexpected(String),
}
