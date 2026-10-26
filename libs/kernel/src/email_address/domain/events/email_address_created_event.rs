//! Domain event raised when a new email address is registered.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::email_address::domain::entities::email_address::EmailAddress;
use crate::email_address::domain::value_objects::email_address_created_at::EmailAddressCreatedAt;
use crate::email_address::domain::value_objects::email_address_id::EmailAddressId;
use crate::email_address::domain::value_objects::email_address_value::EmailAddressValue;

/// Domain event raised when a new [`EmailAddress`] is successfully persisted.
///
/// Built from the aggregate only, so the event's id always belongs to its
/// value.
pub struct EmailAddressCreatedEvent {
    base: DomainEventBase,
    id: EmailAddressId,
    value: EmailAddressValue,
    created_at: EmailAddressCreatedAt,
}

impl EmailAddressCreatedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.email_address.created";

    pub fn new(email_address: &EmailAddress) -> Self {
        Self {
            base: DomainEventBase::new(email_address.id().to_string()),
            id: *email_address.id(),
            value: email_address.value().clone(),
            created_at: *email_address.created_at(),
        }
    }

    pub fn id(&self) -> &EmailAddressId {
        &self.id
    }

    pub fn value(&self) -> &EmailAddressValue {
        &self.value
    }

    pub fn created_at(&self) -> &EmailAddressCreatedAt {
        &self.created_at
    }
}

impl DomainEvent for EmailAddressCreatedEvent {
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
