//! Domain service for finding a single url source.

use std::sync::Arc;

use tracing::debug;

use crate::url_source::domain::entities::url_source::UrlSource;
use crate::url_source::domain::errors::url_source_repository_error::UrlSourceRepositoryError;
use crate::url_source::domain::repositories::url_source_repository::UrlSourceRepository;
use crate::url_source::domain::value_objects::url_source_id::UrlSourceId;

/// Domain service that looks up a single [`UrlSource`] by id.
pub struct UrlSourceFinder {
    repository: Arc<dyn UrlSourceRepository>,
}

impl UrlSourceFinder {
    pub fn new(repository: Arc<dyn UrlSourceRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: UrlSourceId) -> Result<UrlSource, UrlSourceRepositoryError> {
        debug!(id = %id, "Finding url source");
        let url_source = self.repository.find_by_id(&id).await?;

        url_source.ok_or(UrlSourceRepositoryError::NotFound)
    }
}
