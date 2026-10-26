//! Domain service for deleting a DNS resolution.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use crate::dns_resolution::domain::events::create_dns_resolution_deleted_event::create_dns_resolution_deleted_event;
use crate::dns_resolution::domain::repositories::dns_resolution_repository::DnsResolutionRepository;
use crate::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;

/// Domain service that deletes a [`DnsResolution`] with all of its
/// observations and publishes a [`DnsResolutionDeletedEvent`] via the event
/// bus.
///
/// The aggregate is fetched before deletion so the event carries the
/// endpoints of the deleted resolution.
///
/// [`DnsResolution`]: crate::dns_resolution::domain::entities::dns_resolution::DnsResolution
/// [`DnsResolutionDeletedEvent`]: crate::dns_resolution::domain::events::dns_resolution_deleted_event::DnsResolutionDeletedEvent
pub struct DnsResolutionDeleter {
    repository: Arc<dyn DnsResolutionRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl DnsResolutionDeleter {
    pub fn new(repository: Arc<dyn DnsResolutionRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    /// Deletes the resolution `id`.
    ///
    /// # Errors
    ///
    /// Returns [`DnsResolutionRepositoryError::NotFound`] if it is not
    /// recorded, the repository's error when a read or the delete fails, and
    /// [`DnsResolutionRepositoryError::Unexpected`] when publishing fails.
    pub async fn execute(&self, id: DnsResolutionId) -> Result<(), DnsResolutionRepositoryError> {
        debug!(id = %id, "Deleting DNS resolution");

        let dns_resolution = self.repository.find_by_id(&id).await?.ok_or_else(|| {
            warn!(id = %id, "DNS resolution not found for deletion");
            DnsResolutionRepositoryError::NotFound
        })?;

        self.repository.delete(&id).await?;

        let event = create_dns_resolution_deleted_event(&dns_resolution)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| DnsResolutionRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %id, "DNS resolution deleted");
        Ok(())
    }
}
