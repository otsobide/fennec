//! Domain service for updating an existing source.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::source::domain::entities::source::Source;
use crate::source::domain::errors::source_repository_error::SourceRepositoryError;
use crate::source::domain::events::create_source_updated_event::create_source_updated_event;
use crate::source::domain::repositories::source_repository::SourceRepository;
use crate::source::domain::value_objects::source_description::SourceDescription;
use crate::source::domain::value_objects::source_id::SourceId;
use crate::source::domain::value_objects::source_status::SourceStatus;
use crate::source::domain::value_objects::source_updated_at::SourceUpdatedAt;

/// Domain service that updates an existing [`Source`] and publishes
/// a [`SourceUpdatedEvent`] via the event bus.
///
/// Mutates only `status` and `description`. `source_type` is immutable;
/// `created_at` is preserved; `updated_at` is regenerated.
///
/// [`SourceUpdatedEvent`]: crate::source::domain::events::source_updated_event::SourceUpdatedEvent
pub struct SourceUpdater {
    repository: Arc<dyn SourceRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl SourceUpdater {
    pub fn new(repository: Arc<dyn SourceRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(
        &self,
        id: SourceId,
        status: SourceStatus,
        description: SourceDescription,
    ) -> Result<(), SourceRepositoryError> {
        debug!(id = %id, "Updating source");

        let previous = self.repository.find_by_id(&id).await?.ok_or_else(|| {
            warn!(id = %id, "Source not found for update");
            SourceRepositoryError::NotFound
        })?;

        let updated = Source::new(
            id,
            previous.source_type().clone(),
            status,
            description,
            previous.created_at().clone(),
            SourceUpdatedAt::now(),
        );
        self.repository.update(&updated).await?;

        let event = create_source_updated_event(&updated, &previous)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| SourceRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %updated.id(), "Source updated");
        Ok(())
    }
}
