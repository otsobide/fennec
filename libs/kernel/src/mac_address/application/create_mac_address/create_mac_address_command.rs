//! Command for registering a MAC address.

use shared_cqrs::command::domain::command::Command;

use crate::mac_address::domain::value_objects::mac_address_value::MacAddressValue;

/// Command that requests the registration of a MAC address.
///
/// It carries no id: the id of a MAC address is derived from its value by
/// [`MacAddressCreator`].
///
/// [`MacAddressCreator`]: super::mac_address_creator::MacAddressCreator
pub struct CreateMacAddressCommand {
    pub value: MacAddressValue,
}

impl Command for CreateMacAddressCommand {}
