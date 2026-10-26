//! Domain event raised when a new IPv4 network is registered.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::ipv4_network::domain::entities::ipv4_network::Ipv4Network;
use crate::ipv4_network::domain::value_objects::ipv4_network_created_at::Ipv4NetworkCreatedAt;
use crate::ipv4_network::domain::value_objects::ipv4_network_id::Ipv4NetworkId;
use crate::ipv4_network::domain::value_objects::ipv4_network_value::Ipv4NetworkValue;

/// Domain event raised when a new [`Ipv4Network`] is successfully persisted.
///
/// Built from the aggregate only, so the event's id always belongs to its
/// value.
pub struct Ipv4NetworkCreatedEvent {
    base: DomainEventBase,
    id: Ipv4NetworkId,
    value: Ipv4NetworkValue,
    created_at: Ipv4NetworkCreatedAt,
}

impl Ipv4NetworkCreatedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.ipv4_network.created";

    pub fn new(ipv4_network: &Ipv4Network) -> Self {
        Self {
            base: DomainEventBase::new(ipv4_network.id().to_string()),
            id: *ipv4_network.id(),
            value: *ipv4_network.value(),
            created_at: *ipv4_network.created_at(),
        }
    }

    pub fn id(&self) -> &Ipv4NetworkId {
        &self.id
    }

    pub fn value(&self) -> &Ipv4NetworkValue {
        &self.value
    }

    pub fn created_at(&self) -> &Ipv4NetworkCreatedAt {
        &self.created_at
    }
}

impl DomainEvent for Ipv4NetworkCreatedEvent {
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
