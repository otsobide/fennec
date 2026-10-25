//! Domain event raised when an existing sighting is observed again.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::sighting::domain::value_objects::sighting_count::SightingCount;
use crate::sighting::domain::value_objects::sighting_id::SightingId;
use crate::sighting::domain::value_objects::sighting_last_seen::SightingLastSeen;
use crate::sighting::domain::value_objects::sighting_observable_id::SightingObservableId;
use crate::sighting::domain::value_objects::sighting_observable_kind::SightingObservableKind;
use crate::sighting::domain::value_objects::sighting_source_id::SightingSourceId;
use crate::sighting::domain::value_objects::sighting_updated_at::SightingUpdatedAt;

/// Domain event raised when an existing
/// [`Sighting`](crate::sighting::domain::entities::sighting::Sighting) is
/// observed again (its count is bumped and its `last_seen` is refreshed).
pub struct SightingObservedEvent {
    base: DomainEventBase,
    pub id: SightingId,
    pub observable_kind: SightingObservableKind,
    pub observable_id: SightingObservableId,
    pub source_id: SightingSourceId,
    pub observed_at: SystemTime,
    pub new_count: SightingCount,
    pub new_last_seen: SightingLastSeen,
    pub new_updated_at: SightingUpdatedAt,
}

impl SightingObservedEvent {
    pub const EVENT_NAME: &'static str = "fennec.kernel.sighting.observed";

    pub fn new(
        id: SightingId,
        observable_kind: SightingObservableKind,
        observable_id: SightingObservableId,
        source_id: SightingSourceId,
        observed_at: SystemTime,
        new_count: SightingCount,
        new_last_seen: SightingLastSeen,
        new_updated_at: SightingUpdatedAt,
    ) -> Self {
        Self {
            base: DomainEventBase::new(id.to_string()),
            id,
            observable_kind,
            observable_id,
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

    fn to_primitives(&self) -> BTreeMap<String, PrimitiveValue> {
        BTreeMap::from([
            ("id".to_string(), PrimitiveValue::Text(self.id.to_string())),
            (
                "observable_kind".to_string(),
                PrimitiveValue::Text(self.observable_kind.to_string()),
            ),
            (
                "observable_id".to_string(),
                PrimitiveValue::Text(self.observable_id.to_string()),
            ),
            (
                "source_id".to_string(),
                PrimitiveValue::Text(self.source_id.to_string()),
            ),
            (
                "observed_at".to_string(),
                PrimitiveValue::Timestamp(self.observed_at),
            ),
            (
                "new_count".to_string(),
                PrimitiveValue::UnsignedInteger(self.new_count.value()),
            ),
            (
                "new_last_seen".to_string(),
                PrimitiveValue::Timestamp(self.new_last_seen.value()),
            ),
            (
                "new_updated_at".to_string(),
                PrimitiveValue::Timestamp(self.new_updated_at.value()),
            ),
        ])
    }
}
