//! Domain service for registering MAC addresses.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info};

use crate::mac_address::domain::entities::mac_address::MacAddress;
use crate::mac_address::domain::errors::mac_address_repository_error::MacAddressRepositoryError;
use crate::mac_address::domain::events::create_mac_address_created_event::create_mac_address_created_event;
use crate::mac_address::domain::repositories::mac_address_repository::MacAddressRepository;
use crate::mac_address::domain::value_objects::mac_address_created_at::MacAddressCreatedAt;
use crate::mac_address::domain::value_objects::mac_address_value::MacAddressValue;

/// Domain service that persists a new [`MacAddress`] and publishes a
/// [`MacAddressCreatedEvent`] via the event bus.
///
/// The id is derived from the value and `created_at` is generated here, so
/// the caller provides only the address.
///
/// [`MacAddressCreatedEvent`]: crate::mac_address::domain::events::mac_address_created_event::MacAddressCreatedEvent
pub struct MacAddressCreator {
    repository: Arc<dyn MacAddressRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl MacAddressCreator {
    pub fn new(repository: Arc<dyn MacAddressRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(&self, value: MacAddressValue) -> Result<(), MacAddressRepositoryError> {
        let mac_address = MacAddress::new(value, MacAddressCreatedAt::now());
        debug!(id = %mac_address.id(), value = %mac_address.value(), "Creating MAC address");

        self.repository.save(&mac_address).await?;

        let event = create_mac_address_created_event(&mac_address)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| MacAddressRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %mac_address.id(), "MAC address created");
        Ok(())
    }
}
