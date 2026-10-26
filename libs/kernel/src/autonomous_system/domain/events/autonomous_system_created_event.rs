//! Domain event raised when a new autonomous system is registered.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::autonomous_system::domain::entities::autonomous_system::AutonomousSystem;
use crate::autonomous_system::domain::value_objects::autonomous_system_created_at::AutonomousSystemCreatedAt;
use crate::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;
use crate::autonomous_system::domain::value_objects::autonomous_system_value::AutonomousSystemValue;

/// Domain event raised when a new [`AutonomousSystem`] is successfully persisted.
///
/// Built from the aggregate only, so the event's id always belongs to its
/// value.
pub struct AutonomousSystemCreatedEvent {
    base: DomainEventBase,
    id: AutonomousSystemId,
    value: AutonomousSystemValue,
    created_at: AutonomousSystemCreatedAt,
}

impl AutonomousSystemCreatedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.autonomous_system.created";

    pub fn new(autonomous_system: &AutonomousSystem) -> Self {
        Self {
            base: DomainEventBase::new(autonomous_system.id().to_string()),
            id: *autonomous_system.id(),
            value: *autonomous_system.value(),
            created_at: *autonomous_system.created_at(),
        }
    }

    pub fn id(&self) -> &AutonomousSystemId {
        &self.id
    }

    pub fn value(&self) -> &AutonomousSystemValue {
        &self.value
    }

    pub fn created_at(&self) -> &AutonomousSystemCreatedAt {
        &self.created_at
    }
}

impl DomainEvent for AutonomousSystemCreatedEvent {
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
                "value".to_string(),
                PrimitiveValue::Text(self.value.to_string()),
            ),
            (
                "created_at".to_string(),
                PrimitiveValue::Timestamp(self.created_at.value()),
            ),
        ])
    }
}
