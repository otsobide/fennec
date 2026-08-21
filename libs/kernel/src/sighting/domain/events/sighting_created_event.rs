//! Domain event raised when a new sighting is created.

use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};

use crate::sighting::domain::value_objects::sighting_count::SightingCount;
use crate::sighting::domain::value_objects::sighting_created_at::SightingCreatedAt;
use crate::sighting::domain::value_objects::sighting_first_seen::SightingFirstSeen;
use crate::sighting::domain::value_objects::sighting_id::SightingId;
use crate::sighting::domain::value_objects::sighting_ioc_id::SightingIocId;
use crate::sighting::domain::value_objects::sighting_last_seen::SightingLastSeen;
use crate::sighting::domain::value_objects::sighting_source_id::SightingSourceId;
use crate::sighting::domain::value_objects::sighting_updated_at::SightingUpdatedAt;

/// Domain event raised when a new
/// [`Sighting`](crate::sighting::domain::entities::sighting::Sighting) is
/// successfully persisted.
pub struct SightingCreatedEvent {
    base: DomainEventBase,
    pub id: SightingId,
    pub ioc_id: SightingIocId,
    pub source_id: SightingSourceId,
    pub first_seen: SightingFirstSeen,
    pub last_seen: SightingLastSeen,
    pub count: SightingCount,
    pub created_at: SightingCreatedAt,
    pub updated_at: SightingUpdatedAt,
}

impl SightingCreatedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.sighting.sighting.created";

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: SightingId,
        ioc_id: SightingIocId,
        source_id: SightingSourceId,
        first_seen: SightingFirstSeen,
        last_seen: SightingLastSeen,
        count: SightingCount,
        created_at: SightingCreatedAt,
        updated_at: SightingUpdatedAt,
    ) -> Self {
        Self {
            base: DomainEventBase::new(id.to_string()),
            id,
            ioc_id,
            source_id,
            first_seen,
            last_seen,
            count,
            created_at,
            updated_at,
        }
    }
}

impl DomainEvent for SightingCreatedEvent {
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
