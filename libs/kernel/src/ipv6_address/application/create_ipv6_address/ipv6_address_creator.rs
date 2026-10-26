//! Domain service for registering IPv6 addresses.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info};

use crate::ipv6_address::domain::entities::ipv6_address::Ipv6Address;
use crate::ipv6_address::domain::errors::ipv6_address_repository_error::Ipv6AddressRepositoryError;
use crate::ipv6_address::domain::events::create_ipv6_address_created_event::create_ipv6_address_created_event;
use crate::ipv6_address::domain::repositories::ipv6_address_repository::Ipv6AddressRepository;
use crate::ipv6_address::domain::value_objects::ipv6_address_created_at::Ipv6AddressCreatedAt;
use crate::ipv6_address::domain::value_objects::ipv6_address_value::Ipv6AddressValue;

/// Domain service that persists a new [`Ipv6Address`] and publishes an
/// [`Ipv6AddressCreatedEvent`] via the event bus.
///
/// The id is derived from the value and `created_at` is generated here, so
/// the caller provides only the address.
///
/// [`Ipv6AddressCreatedEvent`]: crate::ipv6_address::domain::events::ipv6_address_created_event::Ipv6AddressCreatedEvent
pub struct Ipv6AddressCreator {
    repository: Arc<dyn Ipv6AddressRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl Ipv6AddressCreator {
    pub fn new(repository: Arc<dyn Ipv6AddressRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(&self, value: Ipv6AddressValue) -> Result<(), Ipv6AddressRepositoryError> {
        let ipv6_address = Ipv6Address::new(value, Ipv6AddressCreatedAt::now());
        debug!(id = %ipv6_address.id(), value = %ipv6_address.value(), "Creating IPv6 address");

        self.repository.save(&ipv6_address).await?;

        let event = create_ipv6_address_created_event(&ipv6_address)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| Ipv6AddressRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %ipv6_address.id(), "IPv6 address created");
        Ok(())
    }
}
