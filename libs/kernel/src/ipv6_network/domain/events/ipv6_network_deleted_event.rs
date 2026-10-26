//! Domain event raised when an IPv6 network is deleted.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::ipv6_network::domain::entities::ipv6_network::Ipv6Network;
use crate::ipv6_network::domain::value_objects::ipv6_network_id::Ipv6NetworkId;
use crate::ipv6_network::domain::value_objects::ipv6_network_value::Ipv6NetworkValue;

/// Domain event raised when an [`Ipv6Network`] is deleted.
///
/// Carries the network as well as the id: the id is a one-way hash of the
/// value, so without it a subscriber could not tell which network went away.
/// Built from the aggregate only, so the two always match.
pub struct Ipv6NetworkDeletedEvent {
    base: DomainEventBase,
    id: Ipv6NetworkId,
    value: Ipv6NetworkValue,
}

impl Ipv6NetworkDeletedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.ipv6_network.deleted";

    pub fn new(ipv6_network: &Ipv6Network) -> Self {
        Self {
            base: DomainEventBase::new(ipv6_network.id().to_string()),
            id: *ipv6_network.id(),
            value: *ipv6_network.value(),
        }
    }

    pub fn id(&self) -> &Ipv6NetworkId {
        &self.id
    }

    pub fn value(&self) -> &Ipv6NetworkValue {
        &self.value
    }
}

impl DomainEvent for Ipv6NetworkDeletedEvent {
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
