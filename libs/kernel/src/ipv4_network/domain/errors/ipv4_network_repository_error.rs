//! Error types for IPv4 network repository operations.

use thiserror::Error;

/// Errors that can occur during [`Ipv4NetworkRepository`] operations.
///
/// [`Ipv4NetworkRepository`]: crate::ipv4_network::domain::repositories::ipv4_network_repository::Ipv4NetworkRepository
#[derive(Debug, Error)]
pub enum Ipv4NetworkRepositoryError {
    /// The requested IPv4 network was not found.
    #[error("IPv4 network not found")]
    NotFound,

    /// The IPv4 network is already registered.
    #[error("IPv4 network already exists")]
    AlreadyExists,

    /// An unexpected storage error occurred.
    #[error("unexpected error: {0}")]
    Unexpected(String),
}
