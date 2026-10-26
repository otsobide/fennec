//! Repository trait for the MAC address aggregate.

use async_trait::async_trait;

use crate::mac_address::domain::entities::mac_address::MacAddress;
use crate::mac_address::domain::errors::mac_address_repository_error::MacAddressRepositoryError;
use crate::mac_address::domain::value_objects::mac_address_id::MacAddressId;

/// Async persistence contract for [`MacAddress`] aggregates.
#[async_trait]
pub trait MacAddressRepository: Send + Sync {
    /// Persists a new MAC address.
    ///
    /// # Errors
    ///
    /// Returns [`MacAddressRepositoryError::AlreadyExists`] if the address
    /// is already registered, or [`MacAddressRepositoryError::Unexpected`]
    /// on storage failure.
    async fn save(&self, mac_address: &MacAddress) -> Result<(), MacAddressRepositoryError>;

    /// Retrieves a MAC address by its [`MacAddressId`].
    ///
    /// Returns `Ok(None)` if no MAC address is found.
    async fn find_by_id(
        &self,
        id: &MacAddressId,
    ) -> Result<Option<MacAddress>, MacAddressRepositoryError>;

    /// Deletes a MAC address by its [`MacAddressId`].
    ///
    /// # Errors
    ///
    /// Returns [`MacAddressRepositoryError::NotFound`] if the address is not
    /// registered.
    async fn delete(&self, id: &MacAddressId) -> Result<(), MacAddressRepositoryError>;
}
