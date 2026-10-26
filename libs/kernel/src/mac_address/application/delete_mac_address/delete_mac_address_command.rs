//! Command for deleting a MAC address.

use shared_cqrs::command::domain::command::Command;

use crate::mac_address::domain::value_objects::mac_address_id::MacAddressId;

/// Command that requests the deletion of the MAC address with the given id.
pub struct DeleteMacAddressCommand {
    pub id: MacAddressId,
}

impl Command for DeleteMacAddressCommand {}
