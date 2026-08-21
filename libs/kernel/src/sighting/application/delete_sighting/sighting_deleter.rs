//! Domain service for deleting a sighting.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;
use crate::sighting::domain::events::create_sighting_deleted_event::create_sighting_deleted_event;
use crate::sighting::domain::repositories::sighting_repository::SightingRepository;
use crate::sighting::domain::value_objects::sighting_id::SightingId;

/// Domain service that deletes a [`Sighting`] and publishes a
/// [`SightingDeletedEvent`] via the event bus.
///
/// The aggregate is fetched before deletion so the event carries the
/// `ioc_id` and `source_id` of the deleted sighting.
///
/// [`Sighting`]: crate::sighting::domain::entities::sighting::Sighting
/// [`SightingDeletedEvent`]: crate::sighting::domain::events::sighting_deleted_event::SightingDeletedEvent
pub struct SightingDeleter {
    repository: Arc<dyn SightingRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl SightingDeleter {
    pub fn new(repository: Arc<dyn SightingRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(&self, id: SightingId) -> Result<(), SightingRepositoryError> {
        debug!(id = %id, "Deleting sighting");

        let sighting = self.repository.find_by_id(&id).await?.ok_or_else(|| {
            warn!(id = %id, "Sighting not found for deletion");
            SightingRepositoryError::NotFound
        })?;

        self.repository.delete(&id).await?;

        let event = create_sighting_deleted_event(&sighting)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| SightingRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %id, "Sighting deleted");
        Ok(())
    }
}
