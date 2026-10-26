//! Domain service for deleting a hostname.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::hostname::domain::errors::hostname_repository_error::HostnameRepositoryError;
use crate::hostname::domain::events::create_hostname_deleted_event::create_hostname_deleted_event;
use crate::hostname::domain::repositories::hostname_repository::HostnameRepository;
use crate::hostname::domain::value_objects::hostname_id::HostnameId;

/// Domain service that deletes a [`Hostname`] and publishes a
/// [`HostnameDeletedEvent`] via the event bus.
///
/// [`Hostname`]: crate::hostname::domain::entities::hostname::Hostname
/// [`HostnameDeletedEvent`]: crate::hostname::domain::events::hostname_deleted_event::HostnameDeletedEvent
pub struct HostnameDeleter {
    repository: Arc<dyn HostnameRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl HostnameDeleter {
    pub fn new(repository: Arc<dyn HostnameRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(&self, id: HostnameId) -> Result<(), HostnameRepositoryError> {
        debug!(id = %id, "Deleting hostname");

        let hostname = self.repository.find_by_id(&id).await?.ok_or_else(|| {
            warn!(id = %id, "Hostname not found for deletion");
            HostnameRepositoryError::NotFound
        })?;

        self.repository.delete(&id).await?;

        let event = create_hostname_deleted_event(&hostname)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| HostnameRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %id, "Hostname deleted");
        Ok(())
    }
}
