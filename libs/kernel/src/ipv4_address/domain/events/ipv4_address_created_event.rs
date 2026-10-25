//! Domain event raised when a new ipv4 address is registered.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::ipv4_address::domain::entities::ipv4_address::Ipv4Address;
use crate::ipv4_address::domain::value_objects::ipv4_address_created_at::Ipv4AddressCreatedAt;
use crate::ipv4_address::domain::value_objects::ipv4_address_id::Ipv4AddressId;
use crate::ipv4_address::domain::value_objects::ipv4_address_value::Ipv4AddressValue;

/// Domain event raised when a new [`Ipv4Address`] is successfully persisted.
///
/// Built from the aggregate only, so the event's id always belongs to its
/// value.
pub struct Ipv4AddressCreatedEvent {
    base: DomainEventBase,
    id: Ipv4AddressId,
    value: Ipv4AddressValue,
    created_at: Ipv4AddressCreatedAt,
}

impl Ipv4AddressCreatedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.ipv4_address.created";

    pub fn new(ipv4_address: &Ipv4Address) -> Self {
        Self {
            base: DomainEventBase::new(ipv4_address.id().to_string()),
            id: *ipv4_address.id(),
            value: *ipv4_address.value(),
            created_at: *ipv4_address.created_at(),
        }
    }

    pub fn id(&self) -> &Ipv4AddressId {
        &self.id
    }

    pub fn value(&self) -> &Ipv4AddressValue {
        &self.value
    }

    pub fn created_at(&self) -> &Ipv4AddressCreatedAt {
        &self.created_at
    }
}

impl DomainEvent for Ipv4AddressCreatedEvent {
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
