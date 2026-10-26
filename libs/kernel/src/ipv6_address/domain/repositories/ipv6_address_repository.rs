//! Repository trait for the IPv6 address aggregate.

use async_trait::async_trait;

use crate::ipv6_address::domain::entities::ipv6_address::Ipv6Address;
use crate::ipv6_address::domain::errors::ipv6_address_repository_error::Ipv6AddressRepositoryError;
use crate::ipv6_address::domain::value_objects::ipv6_address_id::Ipv6AddressId;

/// Async persistence contract for [`Ipv6Address`] aggregates.
#[async_trait]
pub trait Ipv6AddressRepository: Send + Sync {
    /// Persists a new IPv6 address.
    ///
    /// # Errors
    ///
    /// Returns [`Ipv6AddressRepositoryError::AlreadyExists`] if the address
    /// is already registered, or [`Ipv6AddressRepositoryError::Unexpected`]
    /// on storage failure.
    async fn save(&self, ipv6_address: &Ipv6Address) -> Result<(), Ipv6AddressRepositoryError>;

    /// Retrieves an IPv6 address by its [`Ipv6AddressId`].
    ///
    /// Returns `Ok(None)` if no IPv6 address is found.
    async fn find_by_id(
        &self,
        id: &Ipv6AddressId,
    ) -> Result<Option<Ipv6Address>, Ipv6AddressRepositoryError>;

    /// Deletes an IPv6 address by its [`Ipv6AddressId`].
    ///
    /// # Errors
    ///
    /// Returns [`Ipv6AddressRepositoryError::NotFound`] if the address is not
    /// registered.
    async fn delete(&self, id: &Ipv6AddressId) -> Result<(), Ipv6AddressRepositoryError>;
}
