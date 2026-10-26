//! Factory function for [`Ipv6AddressDeletedEvent`].

use crate::ipv6_address::domain::entities::ipv6_address::Ipv6Address;
use crate::ipv6_address::domain::errors::ipv6_address_repository_error::Ipv6AddressRepositoryError;
use crate::ipv6_address::domain::events::ipv6_address_deleted_event::Ipv6AddressDeletedEvent;

/// Creates an [`Ipv6AddressDeletedEvent`] from the deleted IPv6 address.
pub fn create_ipv6_address_deleted_event(
    ipv6_address: &Ipv6Address,
) -> Result<Ipv6AddressDeletedEvent, Ipv6AddressRepositoryError> {
    Ok(Ipv6AddressDeletedEvent::new(ipv6_address))
}
