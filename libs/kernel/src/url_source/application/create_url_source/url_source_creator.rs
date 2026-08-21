//! Domain service for creating url sources.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info};

use crate::url_source::domain::entities::url_source::UrlSource;
use crate::url_source::domain::errors::url_source_repository_error::UrlSourceRepositoryError;
use crate::url_source::domain::events::create_url_source_created_event::create_url_source_created_event;
use crate::url_source::domain::repositories::url_source_repository::UrlSourceRepository;
use crate::url_source::domain::value_objects::url_source_created_at::UrlSourceCreatedAt;
use crate::url_source::domain::value_objects::url_source_format::UrlSourceFormat;
use crate::url_source::domain::value_objects::url_source_id::UrlSourceId;
use crate::url_source::domain::value_objects::url_source_polling_interval::UrlSourcePollingInterval;
use crate::url_source::domain::value_objects::url_source_updated_at::UrlSourceUpdatedAt;
use crate::url_source::domain::value_objects::url_source_url::UrlSourceUrl;

/// Domain service that persists a new [`UrlSource`] and publishes a
/// [`UrlSourceCreatedEvent`] via the event bus.
///
/// [`UrlSourceCreatedEvent`]: crate::url_source::domain::events::url_source_created_event::UrlSourceCreatedEvent
pub struct UrlSourceCreator {
    repository: Arc<dyn UrlSourceRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl UrlSourceCreator {
    pub fn new(repository: Arc<dyn UrlSourceRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(
        &self,
        id: UrlSourceId,
        url: UrlSourceUrl,
        format: UrlSourceFormat,
        polling_interval: UrlSourcePollingInterval,
    ) -> Result<(), UrlSourceRepositoryError> {
        let created_at = UrlSourceCreatedAt::now();
        let updated_at = UrlSourceUpdatedAt::from_system_time(created_at.value());

        let url_source = UrlSource::new(id, url, format, polling_interval, created_at, updated_at);
        debug!(id = %url_source.id(), "Creating url source");

        self.repository.save(&url_source).await?;

        let event = create_url_source_created_event(&url_source)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| UrlSourceRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %url_source.id(), "Url source created");
        Ok(())
    }
}
