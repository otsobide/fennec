//! Repository trait for the ioc aggregate.

use async_trait::async_trait;

use crate::ioc::domain::entities::ioc::Ioc;
use crate::ioc::domain::errors::ioc_repository_error::IocRepositoryError;
use crate::ioc::domain::value_objects::ioc_id::IocId;

/// Async persistence contract for [`Ioc`] aggregates.
#[async_trait]
pub trait IocRepository: Send + Sync {
    /// Persists a new ioc.
    ///
    /// # Errors
    ///
    /// Returns [`IocRepositoryError::AlreadyExists`] if an ioc with the same
    /// id already exists, or [`IocRepositoryError::Unexpected`] on storage
    /// failure.
    async fn save(&self, ioc: &Ioc) -> Result<(), IocRepositoryError>;

    /// Retrieves an ioc by its [`IocId`].
    ///
    /// Returns `Ok(None)` if no ioc is found.
    async fn find_by_id(&self, id: &IocId) -> Result<Option<Ioc>, IocRepositoryError>;

    /// Deletes an ioc by its [`IocId`].
    ///
    /// # Errors
    ///
    /// Returns [`IocRepositoryError::NotFound`] if the ioc does not exist.
    async fn delete(&self, id: &IocId) -> Result<(), IocRepositoryError>;
}
