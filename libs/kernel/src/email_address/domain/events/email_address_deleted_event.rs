//! Domain event raised when an email address is deleted.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::email_address::domain::entities::email_address::EmailAddress;
use crate::email_address::domain::value_objects::email_address_id::EmailAddressId;
use crate::email_address::domain::value_objects::email_address_value::EmailAddressValue;

/// Domain event raised when an [`EmailAddress`] is deleted.
///
/// Carries the address as well as the id: the id is a one-way hash of the
/// value, so without it a subscriber could not tell which address went away.
/// Built from the aggregate only, so the two always match.
pub struct EmailAddressDeletedEvent {
    base: DomainEventBase,
    id: EmailAddressId,
    value: EmailAddressValue,
}

impl EmailAddressDeletedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.email_address.deleted";

    pub fn new(email_address: &EmailAddress) -> Self {
        Self {
            base: DomainEventBase::new(email_address.id().to_string()),
            id: *email_address.id(),
            value: email_address.value().clone(),
        }
    }

    pub fn id(&self) -> &EmailAddressId {
        &self.id
    }

    pub fn value(&self) -> &EmailAddressValue {
        &self.value
    }
}

impl DomainEvent for EmailAddressDeletedEvent {
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
