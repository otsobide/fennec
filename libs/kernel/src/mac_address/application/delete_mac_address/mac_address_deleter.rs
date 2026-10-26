//! Domain service for deleting a MAC address.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::mac_address::domain::errors::mac_address_repository_error::MacAddressRepositoryError;
use crate::mac_address::domain::events::create_mac_address_deleted_event::create_mac_address_deleted_event;
use crate::mac_address::domain::repositories::mac_address_repository::MacAddressRepository;
use crate::mac_address::domain::value_objects::mac_address_id::MacAddressId;

/// Domain service that deletes a [`MacAddress`] and publishes a
/// [`MacAddressDeletedEvent`] via the event bus.
///
/// [`MacAddress`]: crate::mac_address::domain::entities::mac_address::MacAddress
/// [`MacAddressDeletedEvent`]: crate::mac_address::domain::events::mac_address_deleted_event::MacAddressDeletedEvent
pub struct MacAddressDeleter {
    repository: Arc<dyn MacAddressRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl MacAddressDeleter {
    pub fn new(repository: Arc<dyn MacAddressRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(&self, id: MacAddressId) -> Result<(), MacAddressRepositoryError> {
        debug!(id = %id, "Deleting MAC address");

        let mac_address = self.repository.find_by_id(&id).await?.ok_or_else(|| {
            warn!(id = %id, "MAC address not found for deletion");
            MacAddressRepositoryError::NotFound
        })?;

        self.repository.delete(&id).await?;

        let event = create_mac_address_deleted_event(&mac_address)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| MacAddressRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %id, "MAC address deleted");
        Ok(())
    }
}
