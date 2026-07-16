//! Domain service for listing sightings by ioc.

use std::sync::Arc;

use tracing::debug;

use crate::sighting::domain::entities::sighting::Sighting;
use crate::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;
use crate::sighting::domain::repositories::sighting_repository::SightingRepository;
use crate::sighting::domain::value_objects::sighting_ioc_id::SightingIocId;

/// Domain service that returns every [`Sighting`] referencing the given ioc.
///
/// Returns an empty vector if no sightings exist for the ioc.
pub struct SightingsByIocLister {
    repository: Arc<dyn SightingRepository>,
}

impl SightingsByIocLister {
    pub fn new(repository: Arc<dyn SightingRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        ioc_id: SightingIocId,
    ) -> Result<Vec<Sighting>, SightingRepositoryError> {
        debug!(ioc_id = %ioc_id, "Listing sightings by ioc");
        self.repository.find_by_ioc_id(&ioc_id).await
    }
}
