//! Repository trait for the hostname aggregate.

use async_trait::async_trait;

use crate::hostname::domain::entities::hostname::Hostname;
use crate::hostname::domain::errors::hostname_repository_error::HostnameRepositoryError;
use crate::hostname::domain::value_objects::hostname_id::HostnameId;

/// Async persistence contract for [`Hostname`] aggregates.
#[async_trait]
pub trait HostnameRepository: Send + Sync {
    /// Persists a new hostname.
    ///
    /// # Errors
    ///
    /// Returns [`HostnameRepositoryError::AlreadyExists`] if the hostname
    /// is already registered, or [`HostnameRepositoryError::Unexpected`]
    /// on storage failure.
    async fn save(&self, hostname: &Hostname) -> Result<(), HostnameRepositoryError>;

    /// Retrieves a hostname by its [`HostnameId`].
    ///
    /// Returns `Ok(None)` if no hostname is found.
    async fn find_by_id(
        &self,
        id: &HostnameId,
    ) -> Result<Option<Hostname>, HostnameRepositoryError>;

    /// Deletes a hostname by its [`HostnameId`].
    ///
    /// # Errors
    ///
    /// Returns [`HostnameRepositoryError::NotFound`] if the hostname is not
    /// registered.
    async fn delete(&self, id: &HostnameId) -> Result<(), HostnameRepositoryError>;
}
