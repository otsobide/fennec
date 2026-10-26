//! Domain service for deleting an IPv4 network.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::ipv4_network::domain::errors::ipv4_network_repository_error::Ipv4NetworkRepositoryError;
use crate::ipv4_network::domain::events::create_ipv4_network_deleted_event::create_ipv4_network_deleted_event;
use crate::ipv4_network::domain::repositories::ipv4_network_repository::Ipv4NetworkRepository;
use crate::ipv4_network::domain::value_objects::ipv4_network_id::Ipv4NetworkId;

/// Domain service that deletes an [`Ipv4Network`] and publishes an
/// [`Ipv4NetworkDeletedEvent`] via the event bus.
///
/// [`Ipv4Network`]: crate::ipv4_network::domain::entities::ipv4_network::Ipv4Network
/// [`Ipv4NetworkDeletedEvent`]: crate::ipv4_network::domain::events::ipv4_network_deleted_event::Ipv4NetworkDeletedEvent
pub struct Ipv4NetworkDeleter {
    repository: Arc<dyn Ipv4NetworkRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl Ipv4NetworkDeleter {
    pub fn new(repository: Arc<dyn Ipv4NetworkRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(&self, id: Ipv4NetworkId) -> Result<(), Ipv4NetworkRepositoryError> {
        debug!(id = %id, "Deleting IPv4 network");

        let ipv4_network = self.repository.find_by_id(&id).await?.ok_or_else(|| {
            warn!(id = %id, "IPv4 network not found for deletion");
            Ipv4NetworkRepositoryError::NotFound
        })?;

        self.repository.delete(&id).await?;

        let event = create_ipv4_network_deleted_event(&ipv4_network)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| Ipv4NetworkRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %id, "IPv4 network deleted");
        Ok(())
    }
}
