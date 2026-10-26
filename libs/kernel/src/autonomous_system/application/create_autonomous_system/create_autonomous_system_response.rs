//! Response for the create-autonomous-system use case.

use crate::autonomous_system::application::find_autonomous_system::find_autonomous_system_response::AutonomousSystemErrorEntry;

/// Response envelope returned by [`CreateAutonomousSystemCommandHandler`].
///
/// [`CreateAutonomousSystemCommandHandler`]: super::create_autonomous_system_command_handler::CreateAutonomousSystemCommandHandler
pub struct CreateAutonomousSystemResponse {
    pub error: Option<AutonomousSystemErrorEntry>,
}
