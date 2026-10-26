//! Command for deleting an IPv4 network.

use shared_cqrs::command::domain::command::Command;

use crate::ipv4_network::domain::value_objects::ipv4_network_id::Ipv4NetworkId;

/// Command that requests the deletion of the IPv4 network with the given id.
pub struct DeleteIpv4NetworkCommand {
    pub id: Ipv4NetworkId,
}

impl Command for DeleteIpv4NetworkCommand {}
