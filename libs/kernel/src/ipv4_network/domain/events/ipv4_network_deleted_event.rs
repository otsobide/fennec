//! Domain event raised when an IPv4 network is deleted.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::ipv4_network::domain::entities::ipv4_network::Ipv4Network;
use crate::ipv4_network::domain::value_objects::ipv4_network_id::Ipv4NetworkId;
use crate::ipv4_network::domain::value_objects::ipv4_network_value::Ipv4NetworkValue;

/// Domain event raised when an [`Ipv4Network`] is deleted.
///
/// Carries the network as well as the id: the id is a one-way hash of the
/// value, so without it a subscriber could not tell which network went away.
/// Built from the aggregate only, so the two always match.
pub struct Ipv4NetworkDeletedEvent {
    base: DomainEventBase,
    id: Ipv4NetworkId,
    value: Ipv4NetworkValue,
}

impl Ipv4NetworkDeletedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.ipv4_network.deleted";

    pub fn new(ipv4_network: &Ipv4Network) -> Self {
        Self {
            base: DomainEventBase::new(ipv4_network.id().to_string()),
            id: *ipv4_network.id(),
            value: *ipv4_network.value(),
        }
    }

    pub fn id(&self) -> &Ipv4NetworkId {
        &self.id
    }

    pub fn value(&self) -> &Ipv4NetworkValue {
        &self.value
    }
}

impl DomainEvent for Ipv4NetworkDeletedEvent {
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
