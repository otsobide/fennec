//! Repository trait for the ipv4 address aggregate.

use async_trait::async_trait;

use crate::ipv4_address::domain::entities::ipv4_address::Ipv4Address;
use crate::ipv4_address::domain::errors::ipv4_address_repository_error::Ipv4AddressRepositoryError;
use crate::ipv4_address::domain::value_objects::ipv4_address_id::Ipv4AddressId;

/// Async persistence contract for [`Ipv4Address`] aggregates.
#[async_trait]
pub trait Ipv4AddressRepository: Send + Sync {
    /// Persists a new ipv4 address.
    ///
    /// # Errors
    ///
    /// Returns [`Ipv4AddressRepositoryError::AlreadyExists`] if the address
    /// is already registered, or [`Ipv4AddressRepositoryError::Unexpected`]
    /// on storage failure.
    async fn save(&self, ipv4_address: &Ipv4Address) -> Result<(), Ipv4AddressRepositoryError>;

    /// Retrieves an ipv4 address by its [`Ipv4AddressId`].
    ///
    /// Returns `Ok(None)` if no ipv4 address is found.
    async fn find_by_id(
        &self,
        id: &Ipv4AddressId,
    ) -> Result<Option<Ipv4Address>, Ipv4AddressRepositoryError>;

    /// Deletes an ipv4 address by its [`Ipv4AddressId`].
    ///
    /// # Errors
    ///
    /// Returns [`Ipv4AddressRepositoryError::NotFound`] if the address is not
    /// registered.
    async fn delete(&self, id: &Ipv4AddressId) -> Result<(), Ipv4AddressRepositoryError>;
}
