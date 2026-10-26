//! Domain event raised when a new hostname is registered.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::hostname::domain::entities::hostname::Hostname;
use crate::hostname::domain::value_objects::hostname_created_at::HostnameCreatedAt;
use crate::hostname::domain::value_objects::hostname_id::HostnameId;
use crate::hostname::domain::value_objects::hostname_value::HostnameValue;

/// Domain event raised when a new [`Hostname`] is successfully persisted.
///
/// Built from the aggregate only, so the event's id always belongs to its
/// value.
pub struct HostnameCreatedEvent {
    base: DomainEventBase,
    id: HostnameId,
    value: HostnameValue,
    created_at: HostnameCreatedAt,
}

impl HostnameCreatedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.hostname.created";

    pub fn new(hostname: &Hostname) -> Self {
        Self {
            base: DomainEventBase::new(hostname.id().to_string()),
            id: *hostname.id(),
            value: hostname.value().clone(),
            created_at: *hostname.created_at(),
        }
    }

    pub fn id(&self) -> &HostnameId {
        &self.id
    }

    pub fn value(&self) -> &HostnameValue {
        &self.value
    }

    pub fn created_at(&self) -> &HostnameCreatedAt {
        &self.created_at
    }
}

impl DomainEvent for HostnameCreatedEvent {
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
