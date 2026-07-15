//! Domain event raised when a new url source is created.

use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};

use crate::url_source::domain::value_objects::url_source_created_at::UrlSourceCreatedAt;
use crate::url_source::domain::value_objects::url_source_format::UrlSourceFormat;
use crate::url_source::domain::value_objects::url_source_id::UrlSourceId;
use crate::url_source::domain::value_objects::url_source_polling_interval::UrlSourcePollingInterval;
use crate::url_source::domain::value_objects::url_source_updated_at::UrlSourceUpdatedAt;
use crate::url_source::domain::value_objects::url_source_url::UrlSourceUrl;

/// Domain event raised when a new [`UrlSource`] is successfully persisted.
///
/// [`UrlSource`]: crate::url_source::domain::entities::url_source::UrlSource
pub struct UrlSourceCreatedEvent {
    base: DomainEventBase,
    pub id: UrlSourceId,
    pub url: UrlSourceUrl,
    pub format: UrlSourceFormat,
    pub polling_interval: UrlSourcePollingInterval,
    pub created_at: UrlSourceCreatedAt,
    pub updated_at: UrlSourceUpdatedAt,
}

impl UrlSourceCreatedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.url_source.url_source.created";

    pub fn new(
        id: UrlSourceId,
        url: UrlSourceUrl,
        format: UrlSourceFormat,
        polling_interval: UrlSourcePollingInterval,
        created_at: UrlSourceCreatedAt,
        updated_at: UrlSourceUpdatedAt,
    ) -> Self {
        Self {
            base: DomainEventBase::new(id.to_string()),
            id,
            url,
            format,
            polling_interval,
            created_at,
            updated_at,
        }
    }
}

impl DomainEvent for UrlSourceCreatedEvent {
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
