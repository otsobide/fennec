//! Domain service for registering hostnames.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info};

use crate::hostname::domain::entities::hostname::Hostname;
use crate::hostname::domain::errors::hostname_repository_error::HostnameRepositoryError;
use crate::hostname::domain::events::create_hostname_created_event::create_hostname_created_event;
use crate::hostname::domain::repositories::hostname_repository::HostnameRepository;
use crate::hostname::domain::value_objects::hostname_created_at::HostnameCreatedAt;
use crate::hostname::domain::value_objects::hostname_value::HostnameValue;

/// Domain service that persists a new [`Hostname`] and publishes a
/// [`HostnameCreatedEvent`] via the event bus.
///
/// The id is derived from the value and `created_at` is generated here, so
/// the caller provides only the hostname.
///
/// [`HostnameCreatedEvent`]: crate::hostname::domain::events::hostname_created_event::HostnameCreatedEvent
pub struct HostnameCreator {
    repository: Arc<dyn HostnameRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl HostnameCreator {
    pub fn new(repository: Arc<dyn HostnameRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(&self, value: HostnameValue) -> Result<(), HostnameRepositoryError> {
        let hostname = Hostname::new(value.clone(), HostnameCreatedAt::now());
        debug!(id = %hostname.id(), value = %hostname.value(), "Creating hostname");

        self.repository.save(&hostname).await?;

        let event = create_hostname_created_event(&hostname)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| HostnameRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %hostname.id(), "Hostname created");
        Ok(())
    }
}
