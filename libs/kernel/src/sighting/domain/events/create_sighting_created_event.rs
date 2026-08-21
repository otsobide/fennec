//! Factory function for [`SightingCreatedEvent`].

use crate::sighting::domain::entities::sighting::Sighting;
use crate::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;
use crate::sighting::domain::events::sighting_created_event::SightingCreatedEvent;

/// Creates a [`SightingCreatedEvent`] from the given sighting.
pub fn create_sighting_created_event(
    sighting: &Sighting,
) -> Result<SightingCreatedEvent, SightingRepositoryError> {
    Ok(SightingCreatedEvent::new(
        sighting.id().clone(),
        sighting.ioc_id().clone(),
        sighting.source_id().clone(),
        sighting.first_seen().clone(),
        sighting.last_seen().clone(),
        *sighting.count(),
        sighting.created_at().clone(),
        sighting.updated_at().clone(),
    ))
}
