//! Domain event raised when a source's observation of a DNS resolution is
//! added or changes.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use crate::dns_resolution::domain::entities::dns_resolution_observation::DnsResolutionObservation;
use crate::dns_resolution::domain::value_objects::dns_resolution_count::DnsResolutionCount;
use crate::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;
use crate::dns_resolution::domain::value_objects::dns_resolution_source_id::DnsResolutionSourceId;
use crate::dns_resolution::domain::value_objects::dns_resolution_updated_at::DnsResolutionUpdatedAt;
use crate::dns_resolution::domain::value_objects::dns_resolution_window::DnsResolutionWindow;

/// Domain event raised when a source's observation of a [`DnsResolution`]
/// is added or widened. A report that changes nothing raises no event.
///
/// Carries the observation as it now stands, so the sequence of these
/// events keeps the history the merged window summarises ("seen in 2019 and
/// again in 2024").
pub struct DnsResolutionObservedEvent {
    base: DomainEventBase,
    id: DnsResolutionId,
    source_id: DnsResolutionSourceId,
    window: DnsResolutionWindow,
    count: DnsResolutionCount,
    updated_at: DnsResolutionUpdatedAt,
}

impl DnsResolutionObservedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.dns_resolution.observed";

    pub fn new(dns_resolution: &DnsResolution, observation: &DnsResolutionObservation) -> Self {
        Self {
            base: DomainEventBase::new(dns_resolution.id().to_string()),
            id: *dns_resolution.id(),
            source_id: observation.source_id().clone(),
            window: *observation.window(),
            count: observation.count(),
            updated_at: *observation.updated_at(),
        }
    }

    pub fn id(&self) -> &DnsResolutionId {
        &self.id
    }

    pub fn source_id(&self) -> &DnsResolutionSourceId {
        &self.source_id
    }
}

impl DomainEvent for DnsResolutionObservedEvent {
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
            (
                "first_seen".to_string(),
                PrimitiveValue::Timestamp(self.window.first_seen()),
            ),
            (
                "last_seen".to_string(),
                PrimitiveValue::Timestamp(self.window.last_seen()),
            ),
            (
                "count".to_string(),
                PrimitiveValue::UnsignedInteger(self.count.value()),
            ),
            (
                "updated_at".to_string(),
                PrimitiveValue::Timestamp(self.updated_at.value()),
            ),
        ])
    }
}
