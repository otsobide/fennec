//! Domain service for creating sources.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info};

use crate::source::domain::entities::source::Source;
use crate::source::domain::errors::source_repository_error::SourceRepositoryError;
use crate::source::domain::events::create_source_created_event::create_source_created_event;
use crate::source::domain::repositories::source_repository::SourceRepository;
use crate::source::domain::value_objects::source_created_at::SourceCreatedAt;
use crate::source::domain::value_objects::source_description::SourceDescription;
use crate::source::domain::value_objects::source_id::SourceId;
use crate::source::domain::value_objects::source_status::SourceStatus;
use crate::source::domain::value_objects::source_type::SourceType;
use crate::source::domain::value_objects::source_updated_at::SourceUpdatedAt;

/// Domain service that persists a new [`Source`] and publishes a
/// [`SourceCreatedEvent`] via the event bus.
///
/// Generates `created_at` and `updated_at` itself — the caller does not
/// provide them.
///
/// [`SourceCreatedEvent`]: crate::source::domain::events::source_created_event::SourceCreatedEvent
pub struct SourceCreator {
    repository: Arc<dyn SourceRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl SourceCreator {
    pub fn new(repository: Arc<dyn SourceRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(
        &self,
        id: SourceId,
        source_type: SourceType,
        status: SourceStatus,
        description: SourceDescription,
    ) -> Result<(), SourceRepositoryError> {
        let created_at = SourceCreatedAt::now();
        let updated_at = SourceUpdatedAt::from_system_time(created_at.value());

        let source = Source::new(id, source_type, status, description, created_at, updated_at);
        debug!(id = %source.id(), "Creating source");

        self.repository.save(&source).await?;

        let event = create_source_created_event(&source)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| SourceRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %source.id(), "Source created");
        Ok(())
    }
}
