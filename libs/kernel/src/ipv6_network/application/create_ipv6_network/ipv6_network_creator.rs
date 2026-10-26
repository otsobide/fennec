//! Domain service for registering IPv6 networks.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info};

use crate::ipv6_network::domain::entities::ipv6_network::Ipv6Network;
use crate::ipv6_network::domain::errors::ipv6_network_repository_error::Ipv6NetworkRepositoryError;
use crate::ipv6_network::domain::events::create_ipv6_network_created_event::create_ipv6_network_created_event;
use crate::ipv6_network::domain::repositories::ipv6_network_repository::Ipv6NetworkRepository;
use crate::ipv6_network::domain::value_objects::ipv6_network_created_at::Ipv6NetworkCreatedAt;
use crate::ipv6_network::domain::value_objects::ipv6_network_value::Ipv6NetworkValue;

/// Domain service that persists a new [`Ipv6Network`] and publishes an
/// [`Ipv6NetworkCreatedEvent`] via the event bus.
///
/// The id is derived from the value and `created_at` is generated here, so
/// the caller provides only the network.
///
/// [`Ipv6NetworkCreatedEvent`]: crate::ipv6_network::domain::events::ipv6_network_created_event::Ipv6NetworkCreatedEvent
pub struct Ipv6NetworkCreator {
    repository: Arc<dyn Ipv6NetworkRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl Ipv6NetworkCreator {
    pub fn new(repository: Arc<dyn Ipv6NetworkRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(&self, value: Ipv6NetworkValue) -> Result<(), Ipv6NetworkRepositoryError> {
        let ipv6_network = Ipv6Network::new(value, Ipv6NetworkCreatedAt::now());
        debug!(id = %ipv6_network.id(), value = %ipv6_network.value(), "Creating IPv6 network");

        self.repository.save(&ipv6_network).await?;

        let event = create_ipv6_network_created_event(&ipv6_network)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| Ipv6NetworkRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %ipv6_network.id(), "IPv6 network created");
        Ok(())
    }
}
