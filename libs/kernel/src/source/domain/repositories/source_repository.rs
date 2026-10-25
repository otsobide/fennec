//! Repository trait for the source aggregate.

use async_trait::async_trait;

use crate::source::domain::entities::source::Source;
use crate::source::domain::errors::source_repository_error::SourceRepositoryError;
use crate::source::domain::value_objects::source_id::SourceId;

/// Async persistence contract for [`Source`] aggregates.
///
/// Concrete implementations are found in the `infrastructure` layer.
#[async_trait]
pub trait SourceRepository: Send + Sync {
    /// Persists a new source.
    ///
    /// # Errors
    ///
    /// Returns [`SourceRepositoryError::AlreadyExists`] if a source with the
    /// same id already exists, or [`SourceRepositoryError::Unexpected`] on
    /// storage failure.
    async fn save(&self, source: &Source) -> Result<(), SourceRepositoryError>;

    /// Retrieves a source by its [`SourceId`].
    ///
    /// Returns `Ok(None)` if no source is found.
    ///
    /// # Errors
    ///
    /// Returns [`SourceRepositoryError::Unexpected`] on storage failure.
    async fn find_by_id(&self, id: &SourceId) -> Result<Option<Source>, SourceRepositoryError>;

    /// Updates an existing source.
    ///
    /// # Errors
    ///
    /// Returns [`SourceRepositoryError::NotFound`] if the source does not
    /// exist, or [`SourceRepositoryError::Unexpected`] on storage failure.
    async fn update(&self, source: &Source) -> Result<(), SourceRepositoryError>;

    /// Deletes a source by its [`SourceId`].
    ///
    /// # Errors
    ///
    /// Returns [`SourceRepositoryError::NotFound`] if the source does not
    /// exist, or [`SourceRepositoryError::Unexpected`] on storage failure.
    async fn delete(&self, id: &SourceId) -> Result<(), SourceRepositoryError>;
}
