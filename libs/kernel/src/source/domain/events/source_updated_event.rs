//! Domain event raised when a source is updated.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::source::domain::value_objects::source_description::SourceDescription;
use crate::source::domain::value_objects::source_id::SourceId;
use crate::source::domain::value_objects::source_status::SourceStatus;
use crate::source::domain::value_objects::source_updated_at::SourceUpdatedAt;

/// Domain event raised when an existing [`Source`] is updated.
///
/// [`Source`]: crate::source::domain::entities::source::Source
pub struct SourceUpdatedEvent {
    base: DomainEventBase,
    pub id: SourceId,
    pub new_status: SourceStatus,
    pub old_status: SourceStatus,
    pub new_description: SourceDescription,
    pub old_description: SourceDescription,
    pub updated_at: SourceUpdatedAt,
}

impl SourceUpdatedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.source.updated";

    /// Creates a new `SourceUpdatedEvent` with auto-generated metadata.
    pub fn new(
        id: SourceId,
        new_status: SourceStatus,
        old_status: SourceStatus,
        new_description: SourceDescription,
        old_description: SourceDescription,
        updated_at: SourceUpdatedAt,
    ) -> Self {
        Self {
            base: DomainEventBase::new(id.to_string()),
            id,
            new_status,
            old_status,
            new_description,
            old_description,
            updated_at,
        }
    }
}

impl DomainEvent for SourceUpdatedEvent {
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
                "new_status".to_string(),
                PrimitiveValue::Text(self.new_status.as_str().to_string()),
            ),
            (
                "old_status".to_string(),
                PrimitiveValue::Text(self.old_status.as_str().to_string()),
            ),
            (
                "new_description".to_string(),
                PrimitiveValue::Text(self.new_description.value().to_string()),
            ),
            (
                "old_description".to_string(),
                PrimitiveValue::Text(self.old_description.value().to_string()),
            ),
            (
                "updated_at".to_string(),
                PrimitiveValue::Timestamp(self.updated_at.value()),
            ),
        ])
    }
}
