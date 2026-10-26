//! Command for registering an autonomous system.

use shared_cqrs::command::domain::command::Command;

use crate::autonomous_system::domain::value_objects::autonomous_system_value::AutonomousSystemValue;

/// Command that requests the registration of an autonomous system.
///
/// It carries no id: the id of an autonomous system is derived from its value by
/// [`AutonomousSystemCreator`].
///
/// [`AutonomousSystemCreator`]: super::autonomous_system_creator::AutonomousSystemCreator
pub struct CreateAutonomousSystemCommand {
    pub value: AutonomousSystemValue,
}

impl Command for CreateAutonomousSystemCommand {}
