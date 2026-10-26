//! Command for deleting an IPv6 network.

use shared_cqrs::command::domain::command::Command;

use crate::ipv6_network::domain::value_objects::ipv6_network_id::Ipv6NetworkId;

/// Command that requests the deletion of the IPv6 network with the given id.
pub struct DeleteIpv6NetworkCommand {
    pub id: Ipv6NetworkId,
}

impl Command for DeleteIpv6NetworkCommand {}
