//! Domain event raised when a url source is deleted.

use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};

use crate::url_source::domain::value_objects::url_source_id::UrlSourceId;

/// Domain event raised when a [`UrlSource`] is deleted.
///
/// [`UrlSource`]: crate::url_source::domain::entities::url_source::UrlSource
pub struct UrlSourceDeletedEvent {
    base: DomainEventBase,
    pub id: UrlSourceId,
}

impl UrlSourceDeletedEvent {
    pub const EVENT_NAME: &'static str = "fennec.url_source.url_source.deleted";

    pub fn new(id: UrlSourceId) -> Self {
        Self {
            base: DomainEventBase::new(id.to_string()),
            id,
        }
    }
}

impl DomainEvent for UrlSourceDeletedEvent {
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
