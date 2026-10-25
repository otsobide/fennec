//! Factory function for [`Ipv4AddressDeletedEvent`].

use crate::ipv4_address::domain::entities::ipv4_address::Ipv4Address;
use crate::ipv4_address::domain::errors::ipv4_address_repository_error::Ipv4AddressRepositoryError;
use crate::ipv4_address::domain::events::ipv4_address_deleted_event::Ipv4AddressDeletedEvent;

/// Creates an [`Ipv4AddressDeletedEvent`] from the deleted ipv4 address.
pub fn create_ipv4_address_deleted_event(
    ipv4_address: &Ipv4Address,
) -> Result<Ipv4AddressDeletedEvent, Ipv4AddressRepositoryError> {
    Ok(Ipv4AddressDeletedEvent::new(ipv4_address))
}
