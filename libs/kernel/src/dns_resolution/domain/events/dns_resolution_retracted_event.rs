//! Domain event raised when a source withdraws its observation of a DNS
//! resolution.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use crate::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;
use crate::dns_resolution::domain::value_objects::dns_resolution_source_id::DnsResolutionSourceId;

/// Domain event raised when a source's observation of a [`DnsResolution`] is
/// removed. When it was the last one, a deleted event follows.
pub struct DnsResolutionRetractedEvent {
    base: DomainEventBase,
    id: DnsResolutionId,
    source_id: DnsResolutionSourceId,
}

impl DnsResolutionRetractedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.dns_resolution.retracted";

    pub fn new(dns_resolution: &DnsResolution, source_id: &DnsResolutionSourceId) -> Self {
        Self {
            base: DomainEventBase::new(dns_resolution.id().to_string()),
            id: *dns_resolution.id(),
            source_id: source_id.clone(),
        }
    }

    pub fn id(&self) -> &DnsResolutionId {
        &self.id
    }

    pub fn source_id(&self) -> &DnsResolutionSourceId {
        &self.source_id
    }
}

impl DomainEvent for DnsResolutionRetractedEvent {
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
                "source_id".to_string(),
                PrimitiveValue::Text(self.source_id.to_string()),
            ),
        ])
    }
}
