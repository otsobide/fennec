//! Domain service for registering IPv4 networks.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info};

use crate::ipv4_network::domain::entities::ipv4_network::Ipv4Network;
use crate::ipv4_network::domain::errors::ipv4_network_repository_error::Ipv4NetworkRepositoryError;
use crate::ipv4_network::domain::events::create_ipv4_network_created_event::create_ipv4_network_created_event;
use crate::ipv4_network::domain::repositories::ipv4_network_repository::Ipv4NetworkRepository;
use crate::ipv4_network::domain::value_objects::ipv4_network_created_at::Ipv4NetworkCreatedAt;
use crate::ipv4_network::domain::value_objects::ipv4_network_value::Ipv4NetworkValue;

/// Domain service that persists a new [`Ipv4Network`] and publishes an
/// [`Ipv4NetworkCreatedEvent`] via the event bus.
///
/// The id is derived from the value and `created_at` is generated here, so
/// the caller provides only the network.
///
/// [`Ipv4NetworkCreatedEvent`]: crate::ipv4_network::domain::events::ipv4_network_created_event::Ipv4NetworkCreatedEvent
pub struct Ipv4NetworkCreator {
    repository: Arc<dyn Ipv4NetworkRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl Ipv4NetworkCreator {
    pub fn new(repository: Arc<dyn Ipv4NetworkRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(&self, value: Ipv4NetworkValue) -> Result<(), Ipv4NetworkRepositoryError> {
        let ipv4_network = Ipv4Network::new(value, Ipv4NetworkCreatedAt::now());
        debug!(id = %ipv4_network.id(), value = %ipv4_network.value(), "Creating IPv4 network");

        self.repository.save(&ipv4_network).await?;

        let event = create_ipv4_network_created_event(&ipv4_network)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| Ipv4NetworkRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %ipv4_network.id(), "IPv4 network created");
        Ok(())
    }
}
