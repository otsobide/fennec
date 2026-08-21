//! Domain service for deleting a url source.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::url_source::domain::errors::url_source_repository_error::UrlSourceRepositoryError;
use crate::url_source::domain::events::create_url_source_deleted_event::create_url_source_deleted_event;
use crate::url_source::domain::repositories::url_source_repository::UrlSourceRepository;
use crate::url_source::domain::value_objects::url_source_id::UrlSourceId;

/// Domain service that deletes a [`UrlSource`] and publishes a
/// [`UrlSourceDeletedEvent`] via the event bus.
///
/// [`UrlSource`]: crate::url_source::domain::entities::url_source::UrlSource
/// [`UrlSourceDeletedEvent`]: crate::url_source::domain::events::url_source_deleted_event::UrlSourceDeletedEvent
pub struct UrlSourceDeleter {
    repository: Arc<dyn UrlSourceRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl UrlSourceDeleter {
    pub fn new(repository: Arc<dyn UrlSourceRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(&self, id: UrlSourceId) -> Result<(), UrlSourceRepositoryError> {
        debug!(id = %id, "Deleting url source");

        let url_source = self.repository.find_by_id(&id).await?.ok_or_else(|| {
            warn!(id = %id, "Url source not found for deletion");
            UrlSourceRepositoryError::NotFound
        })?;

        self.repository.delete(&id).await?;

        let event = create_url_source_deleted_event(&url_source)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| UrlSourceRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %id, "Url source deleted");
        Ok(())
    }
}
