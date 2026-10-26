//! Domain event raised when a new domain name is registered.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::domain_name::domain::entities::domain_name::DomainName;
use crate::domain_name::domain::value_objects::domain_name_created_at::DomainNameCreatedAt;
use crate::domain_name::domain::value_objects::domain_name_id::DomainNameId;
use crate::domain_name::domain::value_objects::domain_name_value::DomainNameValue;

/// Domain event raised when a new [`DomainName`] is successfully persisted.
///
/// Built from the aggregate only, so the event's id always belongs to its
/// value.
pub struct DomainNameCreatedEvent {
    base: DomainEventBase,
    id: DomainNameId,
    value: DomainNameValue,
    created_at: DomainNameCreatedAt,
}

impl DomainNameCreatedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.domain_name.created";

    pub fn new(domain_name: &DomainName) -> Self {
        Self {
            base: DomainEventBase::new(domain_name.id().to_string()),
            id: *domain_name.id(),
            value: domain_name.value().clone(),
            created_at: *domain_name.created_at(),
        }
    }

    pub fn id(&self) -> &DomainNameId {
        &self.id
    }

    pub fn value(&self) -> &DomainNameValue {
        &self.value
    }

    pub fn created_at(&self) -> &DomainNameCreatedAt {
        &self.created_at
    }
}

impl DomainEvent for DomainNameCreatedEvent {
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
