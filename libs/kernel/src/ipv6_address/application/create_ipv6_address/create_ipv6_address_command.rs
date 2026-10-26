//! Command for registering an IPv6 address.

use shared_cqrs::command::domain::command::Command;

use crate::ipv6_address::domain::value_objects::ipv6_address_value::Ipv6AddressValue;

/// Command that requests the registration of an IPv6 address.
///
/// It carries no id: the id of an IPv6 address is derived from its value by
/// [`Ipv6AddressCreator`].
///
/// [`Ipv6AddressCreator`]: super::ipv6_address_creator::Ipv6AddressCreator
pub struct CreateIpv6AddressCommand {
    pub value: Ipv6AddressValue,
}

impl Command for CreateIpv6AddressCommand {}
