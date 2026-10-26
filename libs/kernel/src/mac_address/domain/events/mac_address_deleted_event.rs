//! Domain event raised when a MAC address is deleted.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::mac_address::domain::entities::mac_address::MacAddress;
use crate::mac_address::domain::value_objects::mac_address_id::MacAddressId;
use crate::mac_address::domain::value_objects::mac_address_value::MacAddressValue;

/// Domain event raised when a [`MacAddress`] is deleted.
///
/// Carries the address as well as the id: the id is a one-way hash of the
/// value, so without it a subscriber could not tell which address went away.
/// Built from the aggregate only, so the two always match.
pub struct MacAddressDeletedEvent {
    base: DomainEventBase,
    id: MacAddressId,
    value: MacAddressValue,
}

impl MacAddressDeletedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.mac_address.deleted";

    pub fn new(mac_address: &MacAddress) -> Self {
        Self {
            base: DomainEventBase::new(mac_address.id().to_string()),
            id: *mac_address.id(),
            value: *mac_address.value(),
        }
    }

    pub fn id(&self) -> &MacAddressId {
        &self.id
    }

    pub fn value(&self) -> &MacAddressValue {
        &self.value
    }
}

impl DomainEvent for MacAddressDeletedEvent {
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
