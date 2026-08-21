//! Domain service for deleting a source.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::source::domain::errors::source_repository_error::SourceRepositoryError;
use crate::source::domain::events::create_source_deleted_event::create_source_deleted_event;
use crate::source::domain::repositories::source_repository::SourceRepository;
use crate::source::domain::value_objects::source_id::SourceId;

/// Domain service that deletes a [`Source`] and publishes a
/// [`SourceDeletedEvent`] via the event bus.
///
/// [`Source`]: crate::source::domain::entities::source::Source
/// [`SourceDeletedEvent`]: crate::source::domain::events::source_deleted_event::SourceDeletedEvent
pub struct SourceDeleter {
    repository: Arc<dyn SourceRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl SourceDeleter {
    pub fn new(repository: Arc<dyn SourceRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(&self, id: SourceId) -> Result<(), SourceRepositoryError> {
        debug!(id = %id, "Deleting source");

        let source = self.repository.find_by_id(&id).await?.ok_or_else(|| {
            warn!(id = %id, "Source not found for deletion");
            SourceRepositoryError::NotFound
        })?;

        self.repository.delete(&id).await?;

        let event = create_source_deleted_event(&source)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| SourceRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %id, "Source deleted");
        Ok(())
    }
}
