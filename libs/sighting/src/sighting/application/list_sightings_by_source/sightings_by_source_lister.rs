//! Domain service for listing sightings by source.

use std::sync::Arc;

use tracing::debug;

use crate::sighting::domain::entities::sighting::Sighting;
use crate::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;
use crate::sighting::domain::repositories::sighting_repository::SightingRepository;
use crate::sighting::domain::value_objects::sighting_source_id::SightingSourceId;

/// Domain service that returns every [`Sighting`] reported by the given
/// source.
///
/// Returns an empty vector if no sightings exist for the source.
pub struct SightingsBySourceLister {
    repository: Arc<dyn SightingRepository>,
}

impl SightingsBySourceLister {
    pub fn new(repository: Arc<dyn SightingRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        source_id: SightingSourceId,
    ) -> Result<Vec<Sighting>, SightingRepositoryError> {
        debug!(source_id = %source_id, "Listing sightings by source");
        self.repository.find_by_source_id(&source_id).await
    }
}
