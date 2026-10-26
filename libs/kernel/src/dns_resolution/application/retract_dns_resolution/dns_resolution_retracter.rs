//! Domain service for withdrawing a source's observation of a DNS resolution.

use std::sync::Arc;

use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::dns_resolution::domain::entities::dns_resolution_retraction::DnsResolutionRetraction;
use crate::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use crate::dns_resolution::domain::events::create_dns_resolution_deleted_event::create_dns_resolution_deleted_event;
use crate::dns_resolution::domain::events::create_dns_resolution_retracted_event::create_dns_resolution_retracted_event;
use crate::dns_resolution::domain::repositories::dns_resolution_repository::DnsResolutionRepository;
use crate::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;
use crate::dns_resolution::domain::value_objects::dns_resolution_source_id::DnsResolutionSourceId;

/// Domain service that removes one source's observation from a
/// [`DnsResolution`].
///
/// When other sources still assert the resolution, it is updated and a
/// [`DnsResolutionRetractedEvent`] is published. When the observation was
/// the last one, the resolution is deleted and a
/// [`DnsResolutionRetractedEvent`] then a [`DnsResolutionDeletedEvent`] are
/// published. Nothing is published if the write fails.
///
/// [`DnsResolution`]: crate::dns_resolution::domain::entities::dns_resolution::DnsResolution
/// [`DnsResolutionRetractedEvent`]: crate::dns_resolution::domain::events::dns_resolution_retracted_event::DnsResolutionRetractedEvent
/// [`DnsResolutionDeletedEvent`]: crate::dns_resolution::domain::events::dns_resolution_deleted_event::DnsResolutionDeletedEvent
pub struct DnsResolutionRetracter {
    repository: Arc<dyn DnsResolutionRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl DnsResolutionRetracter {
    pub fn new(repository: Arc<dyn DnsResolutionRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    /// Retracts the observation `source_id` made of the resolution `id`.
    ///
    /// # Errors
    ///
    /// Returns [`DnsResolutionRepositoryError::NotFound`] if the resolution is
    /// not recorded or the source has not observed it, the repository's
    /// error when a read or a write fails, and
    /// [`DnsResolutionRepositoryError::Unexpected`] when publishing fails.
    pub async fn execute(
        &self,
        id: DnsResolutionId,
        source_id: DnsResolutionSourceId,
    ) -> Result<(), DnsResolutionRepositoryError> {
        debug!(id = %id, source_id = %source_id, "Retracting DNS resolution observation");

        let dns_resolution = self.repository.find_by_id(&id).await?.ok_or_else(|| {
            warn!(id = %id, "DNS resolution not found for retraction");
            DnsResolutionRepositoryError::NotFound
        })?;

        match dns_resolution.retract(&source_id) {
            DnsResolutionRetraction::NotObserved => {
                warn!(
                    id = %id,
                    source_id = %source_id,
                    "DNS resolution observation not found for retraction"
                );
                Err(DnsResolutionRepositoryError::NotFound)
            }
            DnsResolutionRetraction::Retracted(updated) => {
                self.repository.update(&updated).await?;

                let retracted = create_dns_resolution_retracted_event(&updated, &source_id)?;
                self.publish(vec![Box::new(retracted)])?;

                info!(id = %id, source_id = %source_id, "DNS resolution observation retracted");
                Ok(())
            }
            DnsResolutionRetraction::LastObservationRetracted => {
                self.repository.delete(&id).await?;

                let retracted = create_dns_resolution_retracted_event(&dns_resolution, &source_id)?;
                let deleted = create_dns_resolution_deleted_event(&dns_resolution)?;
                self.publish(vec![Box::new(retracted), Box::new(deleted)])?;

                info!(
                    id = %id,
                    source_id = %source_id,
                    "Last DNS resolution observation retracted, DNS resolution deleted"
                );
                Ok(())
            }
        }
    }

    fn publish(
        &self,
        events: Vec<Box<dyn DomainEvent>>,
    ) -> Result<(), DnsResolutionRepositoryError> {
        self.event_bus
            .publish(events)
            .map_err(|e| DnsResolutionRepositoryError::Unexpected(e.to_string()))
    }
}
