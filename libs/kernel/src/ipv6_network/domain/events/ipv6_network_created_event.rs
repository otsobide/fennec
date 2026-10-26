//! Domain event raised when a new IPv6 network is registered.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::ipv6_network::domain::entities::ipv6_network::Ipv6Network;
use crate::ipv6_network::domain::value_objects::ipv6_network_created_at::Ipv6NetworkCreatedAt;
use crate::ipv6_network::domain::value_objects::ipv6_network_id::Ipv6NetworkId;
use crate::ipv6_network::domain::value_objects::ipv6_network_value::Ipv6NetworkValue;

/// Domain event raised when a new [`Ipv6Network`] is successfully persisted.
///
/// Built from the aggregate only, so the event's id always belongs to its
/// value.
pub struct Ipv6NetworkCreatedEvent {
    base: DomainEventBase,
    id: Ipv6NetworkId,
    value: Ipv6NetworkValue,
    created_at: Ipv6NetworkCreatedAt,
}

impl Ipv6NetworkCreatedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.ipv6_network.created";

    pub fn new(ipv6_network: &Ipv6Network) -> Self {
        Self {
            base: DomainEventBase::new(ipv6_network.id().to_string()),
            id: *ipv6_network.id(),
            value: *ipv6_network.value(),
            created_at: *ipv6_network.created_at(),
        }
    }

    pub fn id(&self) -> &Ipv6NetworkId {
        &self.id
    }

    pub fn value(&self) -> &Ipv6NetworkValue {
        &self.value
    }

    pub fn created_at(&self) -> &Ipv6NetworkCreatedAt {
        &self.created_at
    }
}

impl DomainEvent for Ipv6NetworkCreatedEvent {
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
