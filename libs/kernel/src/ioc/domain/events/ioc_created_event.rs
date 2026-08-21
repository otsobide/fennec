//! Domain event raised when a new ioc is created.

use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};

use crate::ioc::domain::value_objects::ioc_created_at::IocCreatedAt;
use crate::ioc::domain::value_objects::ioc_id::IocId;
use crate::ioc::domain::value_objects::ioc_type::IocType;
use crate::ioc::domain::value_objects::ioc_updated_at::IocUpdatedAt;
use crate::ioc::domain::value_objects::ioc_value::IocValue;

/// Domain event raised when a new [`Ioc`] is successfully persisted.
///
/// [`Ioc`]: crate::ioc::domain::entities::ioc::Ioc
pub struct IocCreatedEvent {
    base: DomainEventBase,
    pub id: IocId,
    pub ioc_type: IocType,
    pub value: IocValue,
    pub created_at: IocCreatedAt,
    pub updated_at: IocUpdatedAt,
}

impl IocCreatedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.ioc.created";

    pub fn new(
        id: IocId,
        ioc_type: IocType,
        value: IocValue,
        created_at: IocCreatedAt,
        updated_at: IocUpdatedAt,
    ) -> Self {
        Self {
            base: DomainEventBase::new(id.to_string()),
            id,
            ioc_type,
            value,
            created_at,
            updated_at,
        }
    }
}

impl DomainEvent for IocCreatedEvent {
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
}
