use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

pub const TEST_EVENT_NAME: &str = "fennec.test.widget.created";

pub struct TestEvent {
    base: DomainEventBase,
    name: String,
}

#[allow(dead_code)]
impl TestEvent {
    pub fn new(aggregate_id: impl Into<String>, name: &str) -> Self {
        Self {
            base: DomainEventBase::new(aggregate_id),
            name: name.to_string(),
        }
    }
}

impl DomainEvent for TestEvent {
    fn event_name(&self) -> &'static str {
        TEST_EVENT_NAME
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
            (
                "id".to_string(),
                PrimitiveValue::Text(self.base.aggregate_id.clone()),
            ),
            ("name".to_string(), PrimitiveValue::Text(self.name.clone())),
            ("count".to_string(), PrimitiveValue::UnsignedInteger(3)),
            (
                "created_at".to_string(),
                PrimitiveValue::Timestamp(
                    std::time::UNIX_EPOCH + std::time::Duration::from_micros(1_750_000_000_123_456),
                ),
            ),
        ])
    }
}
