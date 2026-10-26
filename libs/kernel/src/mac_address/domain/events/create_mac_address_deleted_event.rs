//! Factory function for [`MacAddressDeletedEvent`].

use crate::mac_address::domain::entities::mac_address::MacAddress;
use crate::mac_address::domain::errors::mac_address_repository_error::MacAddressRepositoryError;
use crate::mac_address::domain::events::mac_address_deleted_event::MacAddressDeletedEvent;

/// Creates a [`MacAddressDeletedEvent`] from the deleted MAC address.
pub fn create_mac_address_deleted_event(
    mac_address: &MacAddress,
) -> Result<MacAddressDeletedEvent, MacAddressRepositoryError> {
    Ok(MacAddressDeletedEvent::new(mac_address))
}
