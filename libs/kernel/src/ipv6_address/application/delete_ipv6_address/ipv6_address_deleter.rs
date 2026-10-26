//! Domain service for deleting an IPv6 address.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::ipv6_address::domain::errors::ipv6_address_repository_error::Ipv6AddressRepositoryError;
use crate::ipv6_address::domain::events::create_ipv6_address_deleted_event::create_ipv6_address_deleted_event;
use crate::ipv6_address::domain::repositories::ipv6_address_repository::Ipv6AddressRepository;
use crate::ipv6_address::domain::value_objects::ipv6_address_id::Ipv6AddressId;

/// Domain service that deletes an [`Ipv6Address`] and publishes an
/// [`Ipv6AddressDeletedEvent`] via the event bus.
///
/// [`Ipv6Address`]: crate::ipv6_address::domain::entities::ipv6_address::Ipv6Address
/// [`Ipv6AddressDeletedEvent`]: crate::ipv6_address::domain::events::ipv6_address_deleted_event::Ipv6AddressDeletedEvent
pub struct Ipv6AddressDeleter {
    repository: Arc<dyn Ipv6AddressRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl Ipv6AddressDeleter {
    pub fn new(repository: Arc<dyn Ipv6AddressRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(&self, id: Ipv6AddressId) -> Result<(), Ipv6AddressRepositoryError> {
        debug!(id = %id, "Deleting IPv6 address");

        let ipv6_address = self.repository.find_by_id(&id).await?.ok_or_else(|| {
            warn!(id = %id, "IPv6 address not found for deletion");
            Ipv6AddressRepositoryError::NotFound
        })?;

        self.repository.delete(&id).await?;

        let event = create_ipv6_address_deleted_event(&ipv6_address)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| Ipv6AddressRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %id, "IPv6 address deleted");
        Ok(())
    }
}
