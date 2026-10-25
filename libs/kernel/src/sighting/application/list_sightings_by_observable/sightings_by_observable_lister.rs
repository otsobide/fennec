//! Domain service for listing the sightings of an observable.

use std::sync::Arc;

use tracing::debug;

use crate::sighting::domain::entities::sighting::Sighting;
use crate::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;
use crate::sighting::domain::repositories::sighting_repository::SightingRepository;
use crate::sighting::domain::value_objects::sighting_observable_id::SightingObservableId;
use crate::sighting::domain::value_objects::sighting_observable_kind::SightingObservableKind;

/// Domain service that returns every [`Sighting`] of the given observable.
///
/// Returns an empty vector if no sightings exist for the observable.
pub struct SightingsByObservableLister {
    repository: Arc<dyn SightingRepository>,
}

impl SightingsByObservableLister {
    pub fn new(repository: Arc<dyn SightingRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        observable_kind: SightingObservableKind,
        observable_id: SightingObservableId,
    ) -> Result<Vec<Sighting>, SightingRepositoryError> {
        debug!(
            observable_kind = %observable_kind,
            observable_id = %observable_id,
            "Listing sightings by observable"
        );
        self.repository
            .find_by_observable(&observable_kind, &observable_id)
            .await
    }
}
