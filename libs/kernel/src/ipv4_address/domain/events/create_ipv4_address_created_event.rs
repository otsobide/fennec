//! Factory function for [`Ipv4AddressCreatedEvent`].

use crate::ipv4_address::domain::entities::ipv4_address::Ipv4Address;
use crate::ipv4_address::domain::errors::ipv4_address_repository_error::Ipv4AddressRepositoryError;
use crate::ipv4_address::domain::events::ipv4_address_created_event::Ipv4AddressCreatedEvent;

/// Creates an [`Ipv4AddressCreatedEvent`] from the given ipv4 address.
pub fn create_ipv4_address_created_event(
    ipv4_address: &Ipv4Address,
) -> Result<Ipv4AddressCreatedEvent, Ipv4AddressRepositoryError> {
    Ok(Ipv4AddressCreatedEvent::new(
        *ipv4_address.id(),
        *ipv4_address.value(),
        *ipv4_address.created_at(),
    ))
}
