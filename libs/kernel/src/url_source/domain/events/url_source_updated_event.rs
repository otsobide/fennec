//! Domain event raised when a url source is updated.

use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};

use crate::url_source::domain::value_objects::url_source_format::UrlSourceFormat;
use crate::url_source::domain::value_objects::url_source_id::UrlSourceId;
use crate::url_source::domain::value_objects::url_source_polling_interval::UrlSourcePollingInterval;
use crate::url_source::domain::value_objects::url_source_updated_at::UrlSourceUpdatedAt;
use crate::url_source::domain::value_objects::url_source_url::UrlSourceUrl;

/// Domain event raised when an existing [`UrlSource`] is updated.
///
/// [`UrlSource`]: crate::url_source::domain::entities::url_source::UrlSource
pub struct UrlSourceUpdatedEvent {
    base: DomainEventBase,
    pub id: UrlSourceId,
    pub new_url: UrlSourceUrl,
    pub old_url: UrlSourceUrl,
    pub new_format: UrlSourceFormat,
    pub old_format: UrlSourceFormat,
    pub new_polling_interval: UrlSourcePollingInterval,
    pub old_polling_interval: UrlSourcePollingInterval,
    pub updated_at: UrlSourceUpdatedAt,
}

impl UrlSourceUpdatedEvent {
    pub const EVENT_NAME: &'static str = "fennec.url_source.url_source.updated";

    pub fn new(
        id: UrlSourceId,
        new_url: UrlSourceUrl,
        old_url: UrlSourceUrl,
        new_format: UrlSourceFormat,
        old_format: UrlSourceFormat,
        new_polling_interval: UrlSourcePollingInterval,
        old_polling_interval: UrlSourcePollingInterval,
        updated_at: UrlSourceUpdatedAt,
    ) -> Self {
        Self {
            base: DomainEventBase::new(id.to_string()),
            id,
            new_url,
            old_url,
            new_format,
            old_format,
            new_polling_interval,
            old_polling_interval,
            updated_at,
        }
    }
}

impl DomainEvent for UrlSourceUpdatedEvent {
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
