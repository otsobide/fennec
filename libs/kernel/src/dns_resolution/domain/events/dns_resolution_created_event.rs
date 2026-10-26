//! Domain event raised when a DNS resolution is recorded for the first time.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use crate::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;
use crate::dns_resolution::domain::value_objects::dns_resolution_created_at::DnsResolutionCreatedAt;
use crate::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;
use crate::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;

/// Domain event raised when a [`DnsResolution`] is recorded for the first
/// time, whichever source reported it first.
///
/// Carries both endpoints with their kind, canonical value and id, so a
/// subscriber can index the link without reading the aggregate. The first
/// observation follows as its own observed event.
pub struct DnsResolutionCreatedEvent {
    base: DomainEventBase,
    id: DnsResolutionId,
    subject: DnsResolutionSubject,
    answer: DnsResolutionAnswer,
    created_at: DnsResolutionCreatedAt,
}

impl DnsResolutionCreatedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.dns_resolution.created";

    pub fn new(dns_resolution: &DnsResolution) -> Self {
        Self {
            base: DomainEventBase::new(dns_resolution.id().to_string()),
            id: *dns_resolution.id(),
            subject: dns_resolution.subject().clone(),
            answer: *dns_resolution.answer(),
            created_at: *dns_resolution.created_at(),
        }
    }

    pub fn id(&self) -> &DnsResolutionId {
        &self.id
    }
}

impl DomainEvent for DnsResolutionCreatedEvent {
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
        let mut primitives = BTreeMap::from([
            ("id".to_string(), PrimitiveValue::Text(self.id.to_string())),
            (
                "subject_kind".to_string(),
                PrimitiveValue::Text(self.subject.kind().to_string()),
            ),
            (
                "subject_value".to_string(),
                PrimitiveValue::Text(self.subject.value().to_string()),
            ),
            (
                "subject_id".to_string(),
                PrimitiveValue::Text(self.subject.id().to_string()),
            ),
            (
                "answer_kind".to_string(),
                PrimitiveValue::Text(self.answer.kind().to_string()),
            ),
            (
                "answer_value".to_string(),
                PrimitiveValue::Text(self.answer.to_string()),
            ),
            (
                "answer_id".to_string(),
                PrimitiveValue::Text(self.answer.id().to_string()),
            ),
        ]);
        primitives.insert(
            "created_at".to_string(),
            PrimitiveValue::Timestamp(self.created_at.value()),
        );
        primitives
    }
}
