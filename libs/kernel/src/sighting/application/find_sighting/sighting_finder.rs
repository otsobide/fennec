//! Domain service for finding a single sighting.

use std::sync::Arc;

use tracing::debug;

use crate::sighting::domain::entities::sighting::Sighting;
use crate::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;
use crate::sighting::domain::repositories::sighting_repository::SightingRepository;
use crate::sighting::domain::value_objects::sighting_id::SightingId;

/// Domain service that looks up a single [`Sighting`] by id.
pub struct SightingFinder {
    repository: Arc<dyn SightingRepository>,
}

impl SightingFinder {
    pub fn new(repository: Arc<dyn SightingRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: SightingId) -> Result<Sighting, SightingRepositoryError> {
        debug!(id = %id, "Finding sighting");
        let sighting = self.repository.find_by_id(&id).await?;

        sighting.ok_or(SightingRepositoryError::NotFound)
    }
}
