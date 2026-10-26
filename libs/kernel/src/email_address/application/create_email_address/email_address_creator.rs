//! Domain service for registering email addresses.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info};

use crate::email_address::domain::entities::email_address::EmailAddress;
use crate::email_address::domain::errors::email_address_repository_error::EmailAddressRepositoryError;
use crate::email_address::domain::events::create_email_address_created_event::create_email_address_created_event;
use crate::email_address::domain::repositories::email_address_repository::EmailAddressRepository;
use crate::email_address::domain::value_objects::email_address_created_at::EmailAddressCreatedAt;
use crate::email_address::domain::value_objects::email_address_value::EmailAddressValue;

/// Domain service that persists a new [`EmailAddress`] and publishes an
/// [`EmailAddressCreatedEvent`] via the event bus.
///
/// The id is derived from the value and `created_at` is generated here, so
/// the caller provides only the address.
///
/// [`EmailAddressCreatedEvent`]: crate::email_address::domain::events::email_address_created_event::EmailAddressCreatedEvent
pub struct EmailAddressCreator {
    repository: Arc<dyn EmailAddressRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl EmailAddressCreator {
    pub fn new(repository: Arc<dyn EmailAddressRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(
        &self,
        value: EmailAddressValue,
    ) -> Result<(), EmailAddressRepositoryError> {
        let email_address = EmailAddress::new(value.clone(), EmailAddressCreatedAt::now());
        debug!(id = %email_address.id(), value = %email_address.value(), "Creating email address");

        self.repository.save(&email_address).await?;

        let event = create_email_address_created_event(&email_address)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| EmailAddressRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %email_address.id(), "Email address created");
        Ok(())
    }
}
