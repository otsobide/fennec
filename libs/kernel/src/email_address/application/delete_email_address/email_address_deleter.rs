//! Domain service for deleting an email address.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::email_address::domain::errors::email_address_repository_error::EmailAddressRepositoryError;
use crate::email_address::domain::events::create_email_address_deleted_event::create_email_address_deleted_event;
use crate::email_address::domain::repositories::email_address_repository::EmailAddressRepository;
use crate::email_address::domain::value_objects::email_address_id::EmailAddressId;

/// Domain service that deletes an [`EmailAddress`] and publishes an
/// [`EmailAddressDeletedEvent`] via the event bus.
///
/// [`EmailAddress`]: crate::email_address::domain::entities::email_address::EmailAddress
/// [`EmailAddressDeletedEvent`]: crate::email_address::domain::events::email_address_deleted_event::EmailAddressDeletedEvent
pub struct EmailAddressDeleter {
    repository: Arc<dyn EmailAddressRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl EmailAddressDeleter {
    pub fn new(repository: Arc<dyn EmailAddressRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(&self, id: EmailAddressId) -> Result<(), EmailAddressRepositoryError> {
        debug!(id = %id, "Deleting email address");

        let email_address = self.repository.find_by_id(&id).await?.ok_or_else(|| {
            warn!(id = %id, "Email address not found for deletion");
            EmailAddressRepositoryError::NotFound
        })?;

        self.repository.delete(&id).await?;

        let event = create_email_address_deleted_event(&email_address)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| EmailAddressRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %id, "Email address deleted");
        Ok(())
    }
}
