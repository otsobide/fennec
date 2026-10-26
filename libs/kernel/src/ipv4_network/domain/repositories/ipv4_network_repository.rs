//! Repository trait for the IPv4 network aggregate.

use async_trait::async_trait;

use crate::ipv4_network::domain::entities::ipv4_network::Ipv4Network;
use crate::ipv4_network::domain::errors::ipv4_network_repository_error::Ipv4NetworkRepositoryError;
use crate::ipv4_network::domain::value_objects::ipv4_network_id::Ipv4NetworkId;

/// Async persistence contract for [`Ipv4Network`] aggregates.
#[async_trait]
pub trait Ipv4NetworkRepository: Send + Sync {
    /// Persists a new IPv4 network.
    ///
    /// # Errors
    ///
    /// Returns [`Ipv4NetworkRepositoryError::AlreadyExists`] if the network
    /// is already registered, or [`Ipv4NetworkRepositoryError::Unexpected`]
    /// on storage failure.
    async fn save(&self, ipv4_network: &Ipv4Network) -> Result<(), Ipv4NetworkRepositoryError>;

    /// Retrieves an IPv4 network by its [`Ipv4NetworkId`].
    ///
    /// Returns `Ok(None)` if no IPv4 network is found.
    async fn find_by_id(
        &self,
        id: &Ipv4NetworkId,
    ) -> Result<Option<Ipv4Network>, Ipv4NetworkRepositoryError>;

    /// Deletes an IPv4 network by its [`Ipv4NetworkId`].
    ///
    /// # Errors
    ///
    /// Returns [`Ipv4NetworkRepositoryError::NotFound`] if the network is not
    /// registered.
    async fn delete(&self, id: &Ipv4NetworkId) -> Result<(), Ipv4NetworkRepositoryError>;
}
