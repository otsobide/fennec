//! Domain event raised when a DNS resolution is deleted.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use crate::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;
use crate::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;
use crate::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;

/// Domain event raised when a [`DnsResolution`] is deleted, with all of its
/// observations: explicitly, or because its last observation was retracted.
///
/// Carries both endpoints, because the id is a one-way hash of them.
pub struct DnsResolutionDeletedEvent {
    base: DomainEventBase,
    id: DnsResolutionId,
    subject: DnsResolutionSubject,
    answer: DnsResolutionAnswer,
}

impl DnsResolutionDeletedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.dns_resolution.deleted";

    pub fn new(dns_resolution: &DnsResolution) -> Self {
        Self {
            base: DomainEventBase::new(dns_resolution.id().to_string()),
            id: *dns_resolution.id(),
            subject: dns_resolution.subject().clone(),
            answer: *dns_resolution.answer(),
        }
    }

    pub fn id(&self) -> &DnsResolutionId {
        &self.id
    }
}

impl DomainEvent for DnsResolutionDeletedEvent {
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
        ])
    }
}
