//! Domain event raised when a source is deleted.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::source::domain::value_objects::source_id::SourceId;

/// Domain event raised when a [`Source`] is deleted.
///
/// [`Source`]: crate::source::domain::entities::source::Source
pub struct SourceDeletedEvent {
    base: DomainEventBase,
    pub id: SourceId,
}

impl SourceDeletedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.source.deleted";

    /// Creates a new `SourceDeletedEvent` with auto-generated metadata.
    pub fn new(id: SourceId) -> Self {
        Self {
            base: DomainEventBase::new(id.to_string()),
            id,
        }
    }
}

impl DomainEvent for SourceDeletedEvent {
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
        BTreeMap::from([("id".to_string(), PrimitiveValue::Text(self.id.to_string()))])
    }
}
