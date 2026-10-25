//! Domain event raised when a sighting is deleted.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::sighting::domain::value_objects::sighting_id::SightingId;
use crate::sighting::domain::value_objects::sighting_observable_id::SightingObservableId;
use crate::sighting::domain::value_objects::sighting_observable_kind::SightingObservableKind;
use crate::sighting::domain::value_objects::sighting_source_id::SightingSourceId;

/// Domain event raised when a
/// [`Sighting`](crate::sighting::domain::entities::sighting::Sighting) is
/// deleted.
pub struct SightingDeletedEvent {
    base: DomainEventBase,
    pub id: SightingId,
    pub observable_kind: SightingObservableKind,
    pub observable_id: SightingObservableId,
    pub source_id: SightingSourceId,
}

impl SightingDeletedEvent {
    pub const EVENT_NAME: &'static str = "fennec.kernel.sighting.deleted";

    pub fn new(
        id: SightingId,
        observable_kind: SightingObservableKind,
        observable_id: SightingObservableId,
        source_id: SightingSourceId,
    ) -> Self {
        Self {
            base: DomainEventBase::new(id.to_string()),
            id,
            observable_kind,
            observable_id,
            source_id,
        }
    }
}

impl DomainEvent for SightingDeletedEvent {
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
        ])
    }
}
