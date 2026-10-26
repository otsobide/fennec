//! Domain event raised when the description of an autonomous system changes.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::autonomous_system::domain::entities::autonomous_system::AutonomousSystem;
use crate::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;
use crate::autonomous_system::domain::value_objects::autonomous_system_name::AutonomousSystemName;
use crate::autonomous_system::domain::value_objects::autonomous_system_rir::AutonomousSystemRir;
use crate::autonomous_system::domain::value_objects::autonomous_system_updated_at::AutonomousSystemUpdatedAt;
use crate::autonomous_system::domain::value_objects::autonomous_system_value::AutonomousSystemValue;

/// Domain event raised when the name or the registry of an
/// [`AutonomousSystem`] is replaced.
///
/// Carries the number next to the id, as the other events of the module do,
/// and both the new and the previous description, so a subscriber can tell
/// what changed without reading the aggregate. An absent name or registry is
/// `null` in the payload.
pub struct AutonomousSystemUpdatedEvent {
    base: DomainEventBase,
    id: AutonomousSystemId,
    value: AutonomousSystemValue,
    new_name: Option<AutonomousSystemName>,
    old_name: Option<AutonomousSystemName>,
    new_rir: Option<AutonomousSystemRir>,
    old_rir: Option<AutonomousSystemRir>,
    updated_at: AutonomousSystemUpdatedAt,
}

impl AutonomousSystemUpdatedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.autonomous_system.updated";

    /// Builds the event from the aggregate after and before the change.
    pub fn new(updated: &AutonomousSystem, previous: &AutonomousSystem) -> Self {
        Self {
            base: DomainEventBase::new(updated.id().to_string()),
            id: *updated.id(),
            value: *updated.value(),
            new_name: updated.name().cloned(),
            old_name: previous.name().cloned(),
            new_rir: updated.rir().copied(),
            old_rir: previous.rir().copied(),
            updated_at: *updated.updated_at(),
        }
    }

    pub fn id(&self) -> &AutonomousSystemId {
        &self.id
    }

    pub fn new_name(&self) -> Option<&AutonomousSystemName> {
        self.new_name.as_ref()
    }

    pub fn old_name(&self) -> Option<&AutonomousSystemName> {
        self.old_name.as_ref()
    }

    pub fn new_rir(&self) -> Option<&AutonomousSystemRir> {
        self.new_rir.as_ref()
    }

    pub fn old_rir(&self) -> Option<&AutonomousSystemRir> {
        self.old_rir.as_ref()
    }
}

impl DomainEvent for AutonomousSystemUpdatedEvent {
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
        let text = |value: Option<String>| value.map_or(PrimitiveValue::Null, PrimitiveValue::Text);
        BTreeMap::from([
            ("id".to_string(), PrimitiveValue::Text(self.id.to_string())),
            (
                "value".to_string(),
                PrimitiveValue::Text(self.value.to_string()),
            ),
            (
                "new_name".to_string(),
                text(self.new_name.as_ref().map(|name| name.value().to_string())),
            ),
            (
                "old_name".to_string(),
                text(self.old_name.as_ref().map(|name| name.value().to_string())),
            ),
            (
                "new_rir".to_string(),
                text(self.new_rir.map(|rir| rir.as_str().to_string())),
            ),
            (
                "old_rir".to_string(),
                text(self.old_rir.map(|rir| rir.as_str().to_string())),
            ),
            (
                "updated_at".to_string(),
                PrimitiveValue::Timestamp(self.updated_at.value()),
            ),
        ])
    }
}
