//! Domain service for creating sightings.

use std::sync::Arc;
use std::time::SystemTime;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info};

use crate::sighting::domain::entities::sighting::Sighting;
use crate::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;
use crate::sighting::domain::events::create_sighting_created_event::create_sighting_created_event;
use crate::sighting::domain::repositories::sighting_repository::SightingRepository;
use crate::sighting::domain::value_objects::sighting_count::SightingCount;
use crate::sighting::domain::value_objects::sighting_created_at::SightingCreatedAt;
use crate::sighting::domain::value_objects::sighting_first_seen::SightingFirstSeen;
use crate::sighting::domain::value_objects::sighting_id::SightingId;
use crate::sighting::domain::value_objects::sighting_ioc_id::SightingIocId;
use crate::sighting::domain::value_objects::sighting_last_seen::SightingLastSeen;
use crate::sighting::domain::value_objects::sighting_source_id::SightingSourceId;
use crate::sighting::domain::value_objects::sighting_updated_at::SightingUpdatedAt;

/// Domain service that persists a new [`Sighting`] and publishes a
/// [`SightingCreatedEvent`] via the event bus.
///
/// [`SightingCreatedEvent`]: crate::sighting::domain::events::sighting_created_event::SightingCreatedEvent
pub struct SightingCreator {
    repository: Arc<dyn SightingRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl SightingCreator {
    pub fn new(
        repository: Arc<dyn SightingRepository>,
        event_bus: Arc<dyn EventBus>,
    ) -> Self {
        Self { repository, event_bus }
    }

    pub async fn execute(
        &self,
        id: SightingId,
        ioc_id: SightingIocId,
        source_id: SightingSourceId,
        observed_at: SystemTime,
    ) -> Result<(), SightingRepositoryError> {
        let first_seen = SightingFirstSeen::from_system_time(observed_at);
        let last_seen = SightingLastSeen::from_system_time(observed_at);
        let count = SightingCount::one();
        let created_at = SightingCreatedAt::now();
        let updated_at = SightingUpdatedAt::from_system_time(created_at.value());

        let sighting = Sighting::new(
            id,
            ioc_id,
            source_id,
            first_seen,
            last_seen,
            count,
            created_at,
            updated_at,
        );
        debug!(id = %sighting.id(), "Creating sighting");

        self.repository.save(&sighting).await?;

        let event = create_sighting_created_event(&sighting)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| SightingRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %sighting.id(), "Sighting created");
        Ok(())
    }
}
