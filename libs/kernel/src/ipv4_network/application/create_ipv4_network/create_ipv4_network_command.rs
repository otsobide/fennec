//! Command for registering an IPv4 network.

use shared_cqrs::command::domain::command::Command;

use crate::ipv4_network::domain::value_objects::ipv4_network_value::Ipv4NetworkValue;

/// Command that requests the registration of an IPv4 network.
///
/// It carries no id: the id of an IPv4 network is derived from its value by
/// [`Ipv4NetworkCreator`].
///
/// [`Ipv4NetworkCreator`]: super::ipv4_network_creator::Ipv4NetworkCreator
pub struct CreateIpv4NetworkCommand {
    pub value: Ipv4NetworkValue,
}

impl Command for CreateIpv4NetworkCommand {}
