//! Error types for MAC address repository operations.

use thiserror::Error;

/// Errors that can occur during [`MacAddressRepository`] operations.
///
/// [`MacAddressRepository`]: crate::mac_address::domain::repositories::mac_address_repository::MacAddressRepository
#[derive(Debug, Error)]
pub enum MacAddressRepositoryError {
    /// The requested MAC address was not found.
    #[error("MAC address not found")]
    NotFound,

    /// The MAC address is already registered.
    #[error("MAC address already exists")]
    AlreadyExists,

    /// An unexpected storage error occurred.
    #[error("unexpected error: {0}")]
    Unexpected(String),
}
