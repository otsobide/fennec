//! Domain event raised when an autonomous system is deleted.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::autonomous_system::domain::entities::autonomous_system::AutonomousSystem;
use crate::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;
use crate::autonomous_system::domain::value_objects::autonomous_system_value::AutonomousSystemValue;

/// Domain event raised when an [`AutonomousSystem`] is deleted.
///
/// Carries the autonomous system as well as the id: the id is a one-way hash of the
/// value, so without it a subscriber could not tell which autonomous system went away.
/// Built from the aggregate only, so the two always match.
pub struct AutonomousSystemDeletedEvent {
    base: DomainEventBase,
    id: AutonomousSystemId,
    value: AutonomousSystemValue,
}

impl AutonomousSystemDeletedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.autonomous_system.deleted";

    pub fn new(autonomous_system: &AutonomousSystem) -> Self {
        Self {
            base: DomainEventBase::new(autonomous_system.id().to_string()),
            id: *autonomous_system.id(),
            value: *autonomous_system.value(),
        }
    }

    pub fn id(&self) -> &AutonomousSystemId {
        &self.id
    }

    pub fn value(&self) -> &AutonomousSystemValue {
        &self.value
    }
}

impl DomainEvent for AutonomousSystemDeletedEvent {
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
        ])
    }
}
