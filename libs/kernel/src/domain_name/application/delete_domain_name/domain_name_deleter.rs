//! Domain service for deleting a domain name.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::domain_name::domain::errors::domain_name_repository_error::DomainNameRepositoryError;
use crate::domain_name::domain::events::create_domain_name_deleted_event::create_domain_name_deleted_event;
use crate::domain_name::domain::repositories::domain_name_repository::DomainNameRepository;
use crate::domain_name::domain::value_objects::domain_name_id::DomainNameId;

/// Domain service that deletes an [`DomainName`] and publishes an
/// [`DomainNameDeletedEvent`] via the event bus.
///
/// [`DomainName`]: crate::domain_name::domain::entities::domain_name::DomainName
/// [`DomainNameDeletedEvent`]: crate::domain_name::domain::events::domain_name_deleted_event::DomainNameDeletedEvent
pub struct DomainNameDeleter {
    repository: Arc<dyn DomainNameRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl DomainNameDeleter {
    pub fn new(repository: Arc<dyn DomainNameRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(&self, id: DomainNameId) -> Result<(), DomainNameRepositoryError> {
        debug!(id = %id, "Deleting domain name");

        let domain_name = self.repository.find_by_id(&id).await?.ok_or_else(|| {
            warn!(id = %id, "Domain name not found for deletion");
            DomainNameRepositoryError::NotFound
        })?;

        self.repository.delete(&id).await?;

        let event = create_domain_name_deleted_event(&domain_name)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| DomainNameRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %id, "Domain name deleted");
        Ok(())
    }
}
