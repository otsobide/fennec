//! Error types for ipv4 address repository operations.

use thiserror::Error;

/// Errors that can occur during [`Ipv4AddressRepository`] operations.
///
/// [`Ipv4AddressRepository`]: crate::ipv4_address::domain::repositories::ipv4_address_repository::Ipv4AddressRepository
#[derive(Debug, Error)]
pub enum Ipv4AddressRepositoryError {
    /// The requested ipv4 address was not found.
    #[error("ipv4 address not found")]
    NotFound,

    /// The ipv4 address is already registered.
    #[error("ipv4 address already exists")]
    AlreadyExists,

    /// An unexpected storage error occurred.
    #[error("unexpected error: {0}")]
    Unexpected(String),
}
