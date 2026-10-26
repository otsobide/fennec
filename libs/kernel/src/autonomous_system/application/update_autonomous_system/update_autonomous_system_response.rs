//! Response for the update-autonomous-system use case.

use crate::autonomous_system::application::find_autonomous_system::find_autonomous_system_response::AutonomousSystemErrorEntry;

/// Response envelope returned by [`UpdateAutonomousSystemCommandHandler`].
///
/// On success, `error` is `None`. On failure, `error` contains the structured error.
///
/// [`UpdateAutonomousSystemCommandHandler`]: super::update_autonomous_system_command_handler::UpdateAutonomousSystemCommandHandler
pub struct UpdateAutonomousSystemResponse {
    pub error: Option<AutonomousSystemErrorEntry>,
}
