//! Repository trait for the url source aggregate.

use async_trait::async_trait;

use crate::url_source::domain::entities::url_source::UrlSource;
use crate::url_source::domain::errors::url_source_repository_error::UrlSourceRepositoryError;
use crate::url_source::domain::value_objects::url_source_id::UrlSourceId;

/// Async persistence contract for [`UrlSource`] aggregates.
#[async_trait]
pub trait UrlSourceRepository: Send + Sync {
    /// Persists a new url source.
    ///
    /// # Errors
    ///
    /// Returns [`UrlSourceRepositoryError::AlreadyExists`] if a url source
    /// with the same id already exists, or
    /// [`UrlSourceRepositoryError::Unexpected`] on storage failure.
    async fn save(&self, url_source: &UrlSource) -> Result<(), UrlSourceRepositoryError>;

    /// Retrieves a url source by its [`UrlSourceId`].
    ///
    /// Returns `Ok(None)` if no url source is found.
    async fn find_by_id(
        &self,
        id: &UrlSourceId,
    ) -> Result<Option<UrlSource>, UrlSourceRepositoryError>;

    /// Updates an existing url source.
    ///
    /// # Errors
    ///
    /// Returns [`UrlSourceRepositoryError::NotFound`] if the url source does
    /// not exist.
    async fn update(&self, url_source: &UrlSource) -> Result<(), UrlSourceRepositoryError>;

    /// Deletes a url source by its [`UrlSourceId`].
    ///
    /// # Errors
    ///
    /// Returns [`UrlSourceRepositoryError::NotFound`] if the url source does
    /// not exist.
    async fn delete(&self, id: &UrlSourceId) -> Result<(), UrlSourceRepositoryError>;
}
