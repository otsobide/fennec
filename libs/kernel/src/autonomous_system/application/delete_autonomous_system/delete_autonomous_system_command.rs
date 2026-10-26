//! Command for deleting an autonomous system.

use shared_cqrs::command::domain::command::Command;

use crate::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;

/// Command that requests the deletion of the autonomous system with the given id.
pub struct DeleteAutonomousSystemCommand {
    pub id: AutonomousSystemId,
}

impl Command for DeleteAutonomousSystemCommand {}
