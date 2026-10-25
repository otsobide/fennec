//! Factory function for [`SightingDeletedEvent`].

use crate::sighting::domain::entities::sighting::Sighting;
use crate::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;
use crate::sighting::domain::events::sighting_deleted_event::SightingDeletedEvent;

/// Creates a [`SightingDeletedEvent`] from the deleted sighting.
pub fn create_sighting_deleted_event(
    sighting: &Sighting,
) -> Result<SightingDeletedEvent, SightingRepositoryError> {
    Ok(SightingDeletedEvent::new(
        sighting.id().clone(),
        sighting.ioc_id().clone(),
        sighting.source_id().clone(),
    ))
}
