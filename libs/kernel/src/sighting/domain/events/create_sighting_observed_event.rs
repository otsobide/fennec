//! Factory function for [`SightingObservedEvent`].

use std::time::SystemTime;

use crate::sighting::domain::entities::sighting::Sighting;
use crate::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;
use crate::sighting::domain::events::sighting_observed_event::SightingObservedEvent;

/// Creates a [`SightingObservedEvent`] from the observed sighting and the raw
/// observation timestamp.
pub fn create_sighting_observed_event(
    sighting: &Sighting,
    observed_at: SystemTime,
) -> Result<SightingObservedEvent, SightingRepositoryError> {
    Ok(SightingObservedEvent::new(
        sighting.id().clone(),
        sighting.ioc_id().clone(),
        sighting.source_id().clone(),
        observed_at,
        *sighting.count(),
        sighting.last_seen().clone(),
        sighting.updated_at().clone(),
    ))
}
