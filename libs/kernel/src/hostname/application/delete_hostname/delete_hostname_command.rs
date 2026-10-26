//! Command for deleting a hostname.

use shared_cqrs::command::domain::command::Command;

use crate::hostname::domain::value_objects::hostname_id::HostnameId;

/// Command that requests the deletion of the hostname with the given id.
pub struct DeleteHostnameCommand {
    pub id: HostnameId,
}

impl Command for DeleteHostnameCommand {}
