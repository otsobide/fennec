//! Domain service for observing an existing sighting.

use std::sync::Arc;
use std::time::SystemTime;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;
use crate::sighting::domain::events::create_sighting_observed_event::create_sighting_observed_event;
use crate::sighting::domain::repositories::sighting_repository::SightingRepository;
use crate::sighting::domain::value_objects::sighting_id::SightingId;
use crate::sighting::domain::value_objects::sighting_updated_at::SightingUpdatedAt;

/// Domain service that records a new observation on an existing [`Sighting`]
/// and publishes a [`SightingObservedEvent`] via the event bus.
///
/// [`Sighting`]: crate::sighting::domain::entities::sighting::Sighting
/// [`SightingObservedEvent`]: crate::sighting::domain::events::sighting_observed_event::SightingObservedEvent
pub struct SightingObserver {
    repository: Arc<dyn SightingRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl SightingObserver {
    pub fn new(repository: Arc<dyn SightingRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(
        &self,
        id: SightingId,
        observed_at: SystemTime,
    ) -> Result<(), SightingRepositoryError> {
        debug!(id = %id, "Observing sighting");

        let previous = self.repository.find_by_id(&id).await?.ok_or_else(|| {
            warn!(id = %id, "Sighting not found for observation");
            SightingRepositoryError::NotFound
        })?;

        let updated = previous.observe(observed_at, SightingUpdatedAt::now());

        self.repository.update(&updated).await?;

        let event = create_sighting_observed_event(&updated, observed_at)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| SightingRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %updated.id(), "Sighting observed");
        Ok(())
    }
}
