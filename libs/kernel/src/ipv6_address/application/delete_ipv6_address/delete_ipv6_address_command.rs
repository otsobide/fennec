//! Command for deleting an IPv6 address.

use shared_cqrs::command::domain::command::Command;

use crate::ipv6_address::domain::value_objects::ipv6_address_id::Ipv6AddressId;

/// Command that requests the deletion of the IPv6 address with the given id.
pub struct DeleteIpv6AddressCommand {
    pub id: Ipv6AddressId,
}

impl Command for DeleteIpv6AddressCommand {}
