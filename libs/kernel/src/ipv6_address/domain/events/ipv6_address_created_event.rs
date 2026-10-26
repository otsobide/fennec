//! Domain event raised when a new IPv6 address is registered.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::ipv6_address::domain::entities::ipv6_address::Ipv6Address;
use crate::ipv6_address::domain::value_objects::ipv6_address_created_at::Ipv6AddressCreatedAt;
use crate::ipv6_address::domain::value_objects::ipv6_address_id::Ipv6AddressId;
use crate::ipv6_address::domain::value_objects::ipv6_address_value::Ipv6AddressValue;

/// Domain event raised when a new [`Ipv6Address`] is successfully persisted.
///
/// Built from the aggregate only, so the event's id always belongs to its
/// value.
pub struct Ipv6AddressCreatedEvent {
    base: DomainEventBase,
    id: Ipv6AddressId,
    value: Ipv6AddressValue,
    created_at: Ipv6AddressCreatedAt,
}

impl Ipv6AddressCreatedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.ipv6_address.created";

    pub fn new(ipv6_address: &Ipv6Address) -> Self {
        Self {
            base: DomainEventBase::new(ipv6_address.id().to_string()),
            id: *ipv6_address.id(),
            value: *ipv6_address.value(),
            created_at: *ipv6_address.created_at(),
        }
    }

    pub fn id(&self) -> &Ipv6AddressId {
        &self.id
    }

    pub fn value(&self) -> &Ipv6AddressValue {
        &self.value
    }

    pub fn created_at(&self) -> &Ipv6AddressCreatedAt {
        &self.created_at
    }
}

impl DomainEvent for Ipv6AddressCreatedEvent {
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
