//! Domain service for updating an existing url source.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::url_source::domain::entities::url_source::UrlSource;
use crate::url_source::domain::errors::url_source_repository_error::UrlSourceRepositoryError;
use crate::url_source::domain::events::create_url_source_updated_event::create_url_source_updated_event;
use crate::url_source::domain::repositories::url_source_repository::UrlSourceRepository;
use crate::url_source::domain::value_objects::url_source_format::UrlSourceFormat;
use crate::url_source::domain::value_objects::url_source_id::UrlSourceId;
use crate::url_source::domain::value_objects::url_source_polling_interval::UrlSourcePollingInterval;
use crate::url_source::domain::value_objects::url_source_updated_at::UrlSourceUpdatedAt;
use crate::url_source::domain::value_objects::url_source_url::UrlSourceUrl;

/// Domain service that updates an existing [`UrlSource`] and publishes a
/// [`UrlSourceUpdatedEvent`] via the event bus.
///
/// Mutates `url`, `format`, `polling_interval`. `created_at` is preserved.
/// `updated_at` is regenerated.
///
/// [`UrlSourceUpdatedEvent`]: crate::url_source::domain::events::url_source_updated_event::UrlSourceUpdatedEvent
pub struct UrlSourceUpdater {
    repository: Arc<dyn UrlSourceRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl UrlSourceUpdater {
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
        debug!(id = %id, "Updating url source");

        let previous = self.repository.find_by_id(&id).await?.ok_or_else(|| {
            warn!(id = %id, "Url source not found for update");
            UrlSourceRepositoryError::NotFound
        })?;

        let updated = UrlSource::new(
            id,
            url,
            format,
            polling_interval,
            previous.created_at().clone(),
            UrlSourceUpdatedAt::now(),
        );
        self.repository.update(&updated).await?;

        let event = create_url_source_updated_event(&updated, &previous)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| UrlSourceRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %updated.id(), "Url source updated");
        Ok(())
    }
}
