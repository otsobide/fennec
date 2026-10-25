//! Domain event raised when a sighting is deleted.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::sighting::domain::value_objects::sighting_id::SightingId;
use crate::sighting::domain::value_objects::sighting_ioc_id::SightingIocId;
use crate::sighting::domain::value_objects::sighting_source_id::SightingSourceId;

/// Domain event raised when a
/// [`Sighting`](crate::sighting::domain::entities::sighting::Sighting) is
/// deleted.
pub struct SightingDeletedEvent {
    base: DomainEventBase,
    pub id: SightingId,
    pub ioc_id: SightingIocId,
    pub source_id: SightingSourceId,
}

impl SightingDeletedEvent {
    pub const EVENT_NAME: &'static str = "fennec.kernel.sighting.deleted";

    pub fn new(id: SightingId, ioc_id: SightingIocId, source_id: SightingSourceId) -> Self {
        Self {
            base: DomainEventBase::new(id.to_string()),
            id,
            ioc_id,
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
                "ioc_id".to_string(),
                PrimitiveValue::Text(self.ioc_id.to_string()),
            ),
            (
                "source_id".to_string(),
                PrimitiveValue::Text(self.source_id.to_string()),
            ),
        ])
    }
}
