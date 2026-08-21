//! Domain event raised when an existing sighting is observed again.

use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};

use crate::sighting::domain::value_objects::sighting_count::SightingCount;
use crate::sighting::domain::value_objects::sighting_id::SightingId;
use crate::sighting::domain::value_objects::sighting_ioc_id::SightingIocId;
use crate::sighting::domain::value_objects::sighting_last_seen::SightingLastSeen;
use crate::sighting::domain::value_objects::sighting_source_id::SightingSourceId;
use crate::sighting::domain::value_objects::sighting_updated_at::SightingUpdatedAt;

/// Domain event raised when an existing
/// [`Sighting`](crate::sighting::domain::entities::sighting::Sighting) is
/// observed again (its count is bumped and its `last_seen` is refreshed).
pub struct SightingObservedEvent {
    base: DomainEventBase,
    pub id: SightingId,
    pub ioc_id: SightingIocId,
    pub source_id: SightingSourceId,
    pub observed_at: SystemTime,
    pub new_count: SightingCount,
    pub new_last_seen: SightingLastSeen,
    pub new_updated_at: SightingUpdatedAt,
}

impl SightingObservedEvent {
    pub const EVENT_NAME: &'static str = "fennec.sighting.sighting.observed";

    pub fn new(
        id: SightingId,
        ioc_id: SightingIocId,
        source_id: SightingSourceId,
        observed_at: SystemTime,
        new_count: SightingCount,
        new_last_seen: SightingLastSeen,
        new_updated_at: SightingUpdatedAt,
    ) -> Self {
        Self {
            base: DomainEventBase::new(id.to_string()),
            id,
            ioc_id,
            source_id,
            observed_at,
            new_count,
            new_last_seen,
            new_updated_at,
        }
    }
}

impl DomainEvent for SightingObservedEvent {
    fn event_name(&self) -> &'static str {
        Self::EVENT_NAME
    }

    fn aggregate_id(&self) -> &str {
        &self.base.aggregate_id
    }

    fn event_id(&self) -> &str {
        &self.base.event_id
    }

    fn occurred_on(&self) -> SystemTime {
        self.base.occurred_on
    }
}
