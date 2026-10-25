//! Command for deleting an ipv4 address.

use shared_cqrs::command::domain::command::Command;

use crate::ipv4_address::domain::value_objects::ipv4_address_id::Ipv4AddressId;

/// Command that requests the deletion of the ipv4 address with the given id.
pub struct DeleteIpv4AddressCommand {
    pub id: Ipv4AddressId,
}

impl Command for DeleteIpv4AddressCommand {}
