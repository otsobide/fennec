//! Domain service for deleting an ipv4 address.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::ipv4_address::domain::errors::ipv4_address_repository_error::Ipv4AddressRepositoryError;
use crate::ipv4_address::domain::events::create_ipv4_address_deleted_event::create_ipv4_address_deleted_event;
use crate::ipv4_address::domain::repositories::ipv4_address_repository::Ipv4AddressRepository;
use crate::ipv4_address::domain::value_objects::ipv4_address_id::Ipv4AddressId;

/// Domain service that deletes an [`Ipv4Address`] and publishes an
/// [`Ipv4AddressDeletedEvent`] via the event bus.
///
/// [`Ipv4Address`]: crate::ipv4_address::domain::entities::ipv4_address::Ipv4Address
/// [`Ipv4AddressDeletedEvent`]: crate::ipv4_address::domain::events::ipv4_address_deleted_event::Ipv4AddressDeletedEvent
pub struct Ipv4AddressDeleter {
    repository: Arc<dyn Ipv4AddressRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl Ipv4AddressDeleter {
    pub fn new(repository: Arc<dyn Ipv4AddressRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(&self, id: Ipv4AddressId) -> Result<(), Ipv4AddressRepositoryError> {
        debug!(id = %id, "Deleting ipv4 address");

        let ipv4_address = self.repository.find_by_id(&id).await?.ok_or_else(|| {
            warn!(id = %id, "Ipv4 address not found for deletion");
            Ipv4AddressRepositoryError::NotFound
        })?;

        self.repository.delete(&id).await?;

        let event = create_ipv4_address_deleted_event(&ipv4_address)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| Ipv4AddressRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %id, "Ipv4 address deleted");
        Ok(())
    }
}
