//! Repository trait for the sighting aggregate.

use async_trait::async_trait;

use crate::sighting::domain::entities::sighting::Sighting;
use crate::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;
use crate::sighting::domain::value_objects::sighting_id::SightingId;
use crate::sighting::domain::value_objects::sighting_ioc_id::SightingIocId;
use crate::sighting::domain::value_objects::sighting_source_id::SightingSourceId;

/// Async persistence contract for [`Sighting`] aggregates.
#[async_trait]
pub trait SightingRepository: Send + Sync {
    /// Persists a new sighting.
    ///
    /// # Errors
    ///
    /// Returns [`SightingRepositoryError::PairAlreadyExists`] if a sighting
    /// for the same `(ioc_id, source_id)` pair already exists,
    /// [`SightingRepositoryError::IdAlreadyExists`] if a sighting with the
    /// same id already exists, or [`SightingRepositoryError::Unexpected`] on
    /// storage failure.
    async fn save(&self, sighting: &Sighting) -> Result<(), SightingRepositoryError>;

    /// Updates an existing sighting.
    ///
    /// # Errors
    ///
    /// Returns [`SightingRepositoryError::NotFound`] if the sighting does not
    /// exist.
    async fn update(&self, sighting: &Sighting) -> Result<(), SightingRepositoryError>;

    /// Retrieves a sighting by its [`SightingId`].
    ///
    /// Returns `Ok(None)` if no sighting is found.
    async fn find_by_id(
        &self,
        id: &SightingId,
    ) -> Result<Option<Sighting>, SightingRepositoryError>;

    /// Retrieves the sighting for the given `(ioc_id, source_id)` pair.
    ///
    /// Returns `Ok(None)` if no matching sighting exists.
    async fn find_by_ioc_and_source(
        &self,
        ioc_id: &SightingIocId,
        source_id: &SightingSourceId,
    ) -> Result<Option<Sighting>, SightingRepositoryError>;

    /// Retrieves every sighting that references the given ioc id.
    ///
    /// Returns an empty vector if no sighting exists for the ioc.
    async fn find_by_ioc_id(
        &self,
        ioc_id: &SightingIocId,
    ) -> Result<Vec<Sighting>, SightingRepositoryError>;

    /// Retrieves every sighting that was reported by the given source id.
    ///
    /// Returns an empty vector if no sighting exists for the source.
    async fn find_by_source_id(
        &self,
        source_id: &SightingSourceId,
    ) -> Result<Vec<Sighting>, SightingRepositoryError>;

    /// Deletes a sighting by its [`SightingId`].
    ///
    /// # Errors
    ///
    /// Returns [`SightingRepositoryError::NotFound`] if the sighting does not
    /// exist.
    async fn delete(&self, id: &SightingId) -> Result<(), SightingRepositoryError>;
}
