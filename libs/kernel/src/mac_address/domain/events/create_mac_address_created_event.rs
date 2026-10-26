//! Factory function for [`MacAddressCreatedEvent`].

use crate::mac_address::domain::entities::mac_address::MacAddress;
use crate::mac_address::domain::errors::mac_address_repository_error::MacAddressRepositoryError;
use crate::mac_address::domain::events::mac_address_created_event::MacAddressCreatedEvent;

/// Creates a [`MacAddressCreatedEvent`] from the given MAC address.
pub fn create_mac_address_created_event(
    mac_address: &MacAddress,
) -> Result<MacAddressCreatedEvent, MacAddressRepositoryError> {
    Ok(MacAddressCreatedEvent::new(mac_address))
}
