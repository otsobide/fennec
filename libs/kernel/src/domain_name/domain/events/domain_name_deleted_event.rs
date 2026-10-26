//! Domain event raised when a domain name is deleted.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::domain_name::domain::entities::domain_name::DomainName;
use crate::domain_name::domain::value_objects::domain_name_id::DomainNameId;
use crate::domain_name::domain::value_objects::domain_name_value::DomainNameValue;

/// Domain event raised when an [`DomainName`] is deleted.
///
/// Carries the name as well as the id: the id is a one-way hash of the
/// value, so without it a subscriber could not tell which name went away.
/// Built from the aggregate only, so the two always match.
pub struct DomainNameDeletedEvent {
    base: DomainEventBase,
    id: DomainNameId,
    value: DomainNameValue,
}

impl DomainNameDeletedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.domain_name.deleted";

    pub fn new(domain_name: &DomainName) -> Self {
        Self {
            base: DomainEventBase::new(domain_name.id().to_string()),
            id: *domain_name.id(),
            value: domain_name.value().clone(),
        }
    }

    pub fn id(&self) -> &DomainNameId {
        &self.id
    }

    pub fn value(&self) -> &DomainNameValue {
        &self.value
    }
}

impl DomainEvent for DomainNameDeletedEvent {
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
