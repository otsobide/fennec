//! Domain service for registering domain namees.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info};

use crate::domain_name::domain::entities::domain_name::DomainName;
use crate::domain_name::domain::errors::domain_name_repository_error::DomainNameRepositoryError;
use crate::domain_name::domain::events::create_domain_name_created_event::create_domain_name_created_event;
use crate::domain_name::domain::repositories::domain_name_repository::DomainNameRepository;
use crate::domain_name::domain::value_objects::domain_name_created_at::DomainNameCreatedAt;
use crate::domain_name::domain::value_objects::domain_name_value::DomainNameValue;

/// Domain service that persists a new [`DomainName`] and publishes an
/// [`DomainNameCreatedEvent`] via the event bus.
///
/// The id is derived from the value and `created_at` is generated here, so
/// the caller provides only the name.
///
/// [`DomainNameCreatedEvent`]: crate::domain_name::domain::events::domain_name_created_event::DomainNameCreatedEvent
pub struct DomainNameCreator {
    repository: Arc<dyn DomainNameRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl DomainNameCreator {
    pub fn new(repository: Arc<dyn DomainNameRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(&self, value: DomainNameValue) -> Result<(), DomainNameRepositoryError> {
        let domain_name = DomainName::new(value.clone(), DomainNameCreatedAt::now());
        debug!(id = %domain_name.id(), value = %domain_name.value(), "Creating domain name");

        self.repository.save(&domain_name).await?;

        let event = create_domain_name_created_event(&domain_name)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| DomainNameRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %domain_name.id(), "Domain name created");
        Ok(())
    }
}
