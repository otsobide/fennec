//! Command for registering an ipv4 address.

use shared_cqrs::command::domain::command::Command;

use crate::ipv4_address::domain::value_objects::ipv4_address_value::Ipv4AddressValue;

/// Command that requests the registration of an ipv4 address.
///
/// It carries no id: the id of an ipv4 address is derived from its value by
/// [`Ipv4AddressCreator`].
///
/// [`Ipv4AddressCreator`]: super::ipv4_address_creator::Ipv4AddressCreator
pub struct CreateIpv4AddressCommand {
    pub value: Ipv4AddressValue,
}

impl Command for CreateIpv4AddressCommand {}
