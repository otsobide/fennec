//! Command for registering a hostname.

use shared_cqrs::command::domain::command::Command;

use crate::hostname::domain::value_objects::hostname_value::HostnameValue;

/// Command that requests the registration of a hostname.
///
/// It carries no id: the id of a hostname is derived from its value by
/// [`HostnameCreator`].
///
/// [`HostnameCreator`]: super::hostname_creator::HostnameCreator
pub struct CreateHostnameCommand {
    pub value: HostnameValue,
}

impl Command for CreateHostnameCommand {}
