//! Domain event raised when an ioc is deleted.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::ioc::domain::value_objects::ioc_id::IocId;

/// Domain event raised when an [`Ioc`] is deleted.
///
/// [`Ioc`]: crate::ioc::domain::entities::ioc::Ioc
pub struct IocDeletedEvent {
    base: DomainEventBase,
    pub id: IocId,
}

impl IocDeletedEvent {
    pub const EVENT_NAME: &'static str = "fennec.kernel.ioc.deleted";

    pub fn new(id: IocId) -> Self {
        Self {
            base: DomainEventBase::new(id.to_string()),
            id,
        }
    }
}

impl DomainEvent for IocDeletedEvent {
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
        BTreeMap::from([("id".to_string(), PrimitiveValue::Text(self.id.to_string()))])
    }
}
