//! Domain service for deleting an IPv6 network.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::ipv6_network::domain::errors::ipv6_network_repository_error::Ipv6NetworkRepositoryError;
use crate::ipv6_network::domain::events::create_ipv6_network_deleted_event::create_ipv6_network_deleted_event;
use crate::ipv6_network::domain::repositories::ipv6_network_repository::Ipv6NetworkRepository;
use crate::ipv6_network::domain::value_objects::ipv6_network_id::Ipv6NetworkId;

/// Domain service that deletes an [`Ipv6Network`] and publishes an
/// [`Ipv6NetworkDeletedEvent`] via the event bus.
///
/// [`Ipv6Network`]: crate::ipv6_network::domain::entities::ipv6_network::Ipv6Network
/// [`Ipv6NetworkDeletedEvent`]: crate::ipv6_network::domain::events::ipv6_network_deleted_event::Ipv6NetworkDeletedEvent
pub struct Ipv6NetworkDeleter {
    repository: Arc<dyn Ipv6NetworkRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl Ipv6NetworkDeleter {
    pub fn new(repository: Arc<dyn Ipv6NetworkRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(&self, id: Ipv6NetworkId) -> Result<(), Ipv6NetworkRepositoryError> {
        debug!(id = %id, "Deleting IPv6 network");

        let ipv6_network = self.repository.find_by_id(&id).await?.ok_or_else(|| {
            warn!(id = %id, "IPv6 network not found for deletion");
            Ipv6NetworkRepositoryError::NotFound
        })?;

        self.repository.delete(&id).await?;

        let event = create_ipv6_network_deleted_event(&ipv6_network)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| Ipv6NetworkRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %id, "IPv6 network deleted");
        Ok(())
    }
}
