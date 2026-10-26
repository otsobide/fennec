//! Command for registering an IPv6 network.

use shared_cqrs::command::domain::command::Command;

use crate::ipv6_network::domain::value_objects::ipv6_network_value::Ipv6NetworkValue;

/// Command that requests the registration of an IPv6 network.
///
/// It carries no id: the id of an IPv6 network is derived from its value by
/// [`Ipv6NetworkCreator`].
///
/// [`Ipv6NetworkCreator`]: super::ipv6_network_creator::Ipv6NetworkCreator
pub struct CreateIpv6NetworkCommand {
    pub value: Ipv6NetworkValue,
}

impl Command for CreateIpv6NetworkCommand {}
