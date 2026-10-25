//! Repository trait for the sighting aggregate.

use async_trait::async_trait;

use crate::sighting::domain::entities::sighting::Sighting;
use crate::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;
use crate::sighting::domain::value_objects::sighting_id::SightingId;
use crate::sighting::domain::value_objects::sighting_observable_id::SightingObservableId;
use crate::sighting::domain::value_objects::sighting_observable_kind::SightingObservableKind;
use crate::sighting::domain::value_objects::sighting_source_id::SightingSourceId;

/// Async persistence contract for [`Sighting`] aggregates.
#[async_trait]
pub trait SightingRepository: Send + Sync {
    /// Persists a new sighting.
    ///
    /// # Errors
    ///
    /// Returns [`SightingRepositoryError::PairAlreadyExists`] if a sighting
    /// for the same `(observable_kind, observable_id, source_id)` triple already exists,
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

    /// Retrieves the sighting for the given `(observable_kind, observable_id, source_id)` triple.
    ///
    /// Returns `Ok(None)` if no matching sighting exists.
    async fn find_by_observable_and_source(
        &self,
        observable_kind: &SightingObservableKind,
        observable_id: &SightingObservableId,
        source_id: &SightingSourceId,
    ) -> Result<Option<Sighting>, SightingRepositoryError>;

    /// Retrieves every sighting that references the given observable.
    ///
    /// Returns an empty vector if no sighting exists for the observable.
    async fn find_by_observable(
        &self,
        observable_kind: &SightingObservableKind,
        observable_id: &SightingObservableId,
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
