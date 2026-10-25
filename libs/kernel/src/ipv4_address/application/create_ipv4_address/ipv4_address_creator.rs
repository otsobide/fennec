//! Domain service for registering ipv4 addresses.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info};

use crate::ipv4_address::domain::entities::ipv4_address::Ipv4Address;
use crate::ipv4_address::domain::errors::ipv4_address_repository_error::Ipv4AddressRepositoryError;
use crate::ipv4_address::domain::events::create_ipv4_address_created_event::create_ipv4_address_created_event;
use crate::ipv4_address::domain::repositories::ipv4_address_repository::Ipv4AddressRepository;
use crate::ipv4_address::domain::value_objects::ipv4_address_created_at::Ipv4AddressCreatedAt;
use crate::ipv4_address::domain::value_objects::ipv4_address_value::Ipv4AddressValue;

/// Domain service that persists a new [`Ipv4Address`] and publishes an
/// [`Ipv4AddressCreatedEvent`] via the event bus.
///
/// The id is derived from the value and `created_at` is generated here, so
/// the caller provides only the address.
///
/// [`Ipv4AddressCreatedEvent`]: crate::ipv4_address::domain::events::ipv4_address_created_event::Ipv4AddressCreatedEvent
pub struct Ipv4AddressCreator {
    repository: Arc<dyn Ipv4AddressRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl Ipv4AddressCreator {
    pub fn new(repository: Arc<dyn Ipv4AddressRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(&self, value: Ipv4AddressValue) -> Result<(), Ipv4AddressRepositoryError> {
        let ipv4_address = Ipv4Address::new(value, Ipv4AddressCreatedAt::now());
        debug!(id = %ipv4_address.id(), value = %ipv4_address.value(), "Creating ipv4 address");

        self.repository.save(&ipv4_address).await?;

        let event = create_ipv4_address_created_event(&ipv4_address)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| Ipv4AddressRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %ipv4_address.id(), "Ipv4 address created");
        Ok(())
    }
}
