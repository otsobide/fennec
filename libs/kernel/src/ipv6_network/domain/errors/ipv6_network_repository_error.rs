//! Error types for IPv6 network repository operations.

use thiserror::Error;

/// Errors that can occur during [`Ipv6NetworkRepository`] operations.
///
/// [`Ipv6NetworkRepository`]: crate::ipv6_network::domain::repositories::ipv6_network_repository::Ipv6NetworkRepository
#[derive(Debug, Error)]
pub enum Ipv6NetworkRepositoryError {
    /// The requested IPv6 network was not found.
    #[error("IPv6 network not found")]
    NotFound,

    /// The IPv6 network is already registered.
    #[error("IPv6 network already exists")]
    AlreadyExists,

    /// An unexpected storage error occurred.
    #[error("unexpected error: {0}")]
    Unexpected(String),
}
