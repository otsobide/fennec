//! Factory function for [`Ipv6AddressCreatedEvent`].

use crate::ipv6_address::domain::entities::ipv6_address::Ipv6Address;
use crate::ipv6_address::domain::errors::ipv6_address_repository_error::Ipv6AddressRepositoryError;
use crate::ipv6_address::domain::events::ipv6_address_created_event::Ipv6AddressCreatedEvent;

/// Creates an [`Ipv6AddressCreatedEvent`] from the given IPv6 address.
pub fn create_ipv6_address_created_event(
    ipv6_address: &Ipv6Address,
) -> Result<Ipv6AddressCreatedEvent, Ipv6AddressRepositoryError> {
    Ok(Ipv6AddressCreatedEvent::new(ipv6_address))
}
