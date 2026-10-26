//! Domain event raised when a new MAC address is registered.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::mac_address::domain::entities::mac_address::MacAddress;
use crate::mac_address::domain::value_objects::mac_address_created_at::MacAddressCreatedAt;
use crate::mac_address::domain::value_objects::mac_address_id::MacAddressId;
use crate::mac_address::domain::value_objects::mac_address_value::MacAddressValue;

/// Domain event raised when a new [`MacAddress`] is successfully persisted.
///
/// Built from the aggregate only, so the event's id always belongs to its
/// value.
pub struct MacAddressCreatedEvent {
    base: DomainEventBase,
    id: MacAddressId,
    value: MacAddressValue,
    created_at: MacAddressCreatedAt,
}

impl MacAddressCreatedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.mac_address.created";

    pub fn new(mac_address: &MacAddress) -> Self {
        Self {
            base: DomainEventBase::new(mac_address.id().to_string()),
            id: *mac_address.id(),
            value: *mac_address.value(),
            created_at: *mac_address.created_at(),
        }
    }

    pub fn id(&self) -> &MacAddressId {
        &self.id
    }

    pub fn value(&self) -> &MacAddressValue {
        &self.value
    }

    pub fn created_at(&self) -> &MacAddressCreatedAt {
        &self.created_at
    }
}

impl DomainEvent for MacAddressCreatedEvent {
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
