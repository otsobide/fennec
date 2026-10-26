//! Error types for IPv6 address repository operations.

use thiserror::Error;

/// Errors that can occur during [`Ipv6AddressRepository`] operations.
///
/// [`Ipv6AddressRepository`]: crate::ipv6_address::domain::repositories::ipv6_address_repository::Ipv6AddressRepository
#[derive(Debug, Error)]
pub enum Ipv6AddressRepositoryError {
    /// The requested IPv6 address was not found.
    #[error("IPv6 address not found")]
    NotFound,

    /// The IPv6 address is already registered.
    #[error("IPv6 address already exists")]
    AlreadyExists,

    /// An unexpected storage error occurred.
    #[error("unexpected error: {0}")]
    Unexpected(String),
}
