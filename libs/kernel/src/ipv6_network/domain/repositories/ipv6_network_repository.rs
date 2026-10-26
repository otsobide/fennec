//! Repository trait for the IPv6 network aggregate.

use async_trait::async_trait;

use crate::ipv6_network::domain::entities::ipv6_network::Ipv6Network;
use crate::ipv6_network::domain::errors::ipv6_network_repository_error::Ipv6NetworkRepositoryError;
use crate::ipv6_network::domain::value_objects::ipv6_network_id::Ipv6NetworkId;

/// Async persistence contract for [`Ipv6Network`] aggregates.
#[async_trait]
pub trait Ipv6NetworkRepository: Send + Sync {
    /// Persists a new IPv6 network.
    ///
    /// # Errors
    ///
    /// Returns [`Ipv6NetworkRepositoryError::AlreadyExists`] if the network
    /// is already registered, or [`Ipv6NetworkRepositoryError::Unexpected`]
    /// on storage failure.
    async fn save(&self, ipv6_network: &Ipv6Network) -> Result<(), Ipv6NetworkRepositoryError>;

    /// Retrieves an IPv6 network by its [`Ipv6NetworkId`].
    ///
    /// Returns `Ok(None)` if no IPv6 network is found.
    async fn find_by_id(
        &self,
        id: &Ipv6NetworkId,
    ) -> Result<Option<Ipv6Network>, Ipv6NetworkRepositoryError>;

    /// Deletes an IPv6 network by its [`Ipv6NetworkId`].
    ///
    /// # Errors
    ///
    /// Returns [`Ipv6NetworkRepositoryError::NotFound`] if the network is not
    /// registered.
    async fn delete(&self, id: &Ipv6NetworkId) -> Result<(), Ipv6NetworkRepositoryError>;
}
