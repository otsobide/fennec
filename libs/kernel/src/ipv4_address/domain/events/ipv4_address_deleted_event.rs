//! Domain event raised when an ipv4 address is deleted.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::ipv4_address::domain::entities::ipv4_address::Ipv4Address;
use crate::ipv4_address::domain::value_objects::ipv4_address_id::Ipv4AddressId;
use crate::ipv4_address::domain::value_objects::ipv4_address_value::Ipv4AddressValue;

/// Domain event raised when an [`Ipv4Address`] is deleted.
///
/// Carries the address as well as the id: the id is a one-way hash of the
/// value, so without it a subscriber could not tell which address went away.
/// Built from the aggregate only, so the two always match.
pub struct Ipv4AddressDeletedEvent {
    base: DomainEventBase,
    id: Ipv4AddressId,
    value: Ipv4AddressValue,
}

impl Ipv4AddressDeletedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.ipv4_address.deleted";

    pub fn new(ipv4_address: &Ipv4Address) -> Self {
        Self {
            base: DomainEventBase::new(ipv4_address.id().to_string()),
            id: *ipv4_address.id(),
            value: *ipv4_address.value(),
        }
    }

    pub fn id(&self) -> &Ipv4AddressId {
        &self.id
    }

    pub fn value(&self) -> &Ipv4AddressValue {
        &self.value
    }
}

impl DomainEvent for Ipv4AddressDeletedEvent {
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
