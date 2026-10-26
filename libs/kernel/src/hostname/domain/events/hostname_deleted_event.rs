//! Domain event raised when a hostname is deleted.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::hostname::domain::entities::hostname::Hostname;
use crate::hostname::domain::value_objects::hostname_id::HostnameId;
use crate::hostname::domain::value_objects::hostname_value::HostnameValue;

/// Domain event raised when a [`Hostname`] is deleted.
///
/// Carries the hostname as well as the id: the id is a one-way hash of the
/// value, so without it a subscriber could not tell which hostname went away.
/// Built from the aggregate only, so the two always match.
pub struct HostnameDeletedEvent {
    base: DomainEventBase,
    id: HostnameId,
    value: HostnameValue,
}

impl HostnameDeletedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.hostname.deleted";

    pub fn new(hostname: &Hostname) -> Self {
        Self {
            base: DomainEventBase::new(hostname.id().to_string()),
            id: *hostname.id(),
            value: hostname.value().clone(),
        }
    }

    pub fn id(&self) -> &HostnameId {
        &self.id
    }

    pub fn value(&self) -> &HostnameValue {
        &self.value
    }
}

impl DomainEvent for HostnameDeletedEvent {
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
