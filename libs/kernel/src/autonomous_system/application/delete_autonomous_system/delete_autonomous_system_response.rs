//! Response for the delete-autonomous-system use case.

use crate::autonomous_system::application::find_autonomous_system::find_autonomous_system_response::AutonomousSystemErrorEntry;

/// Response envelope returned by [`DeleteAutonomousSystemCommandHandler`].
///
/// [`DeleteAutonomousSystemCommandHandler`]: super::delete_autonomous_system_command_handler::DeleteAutonomousSystemCommandHandler
pub struct DeleteAutonomousSystemResponse {
    pub error: Option<AutonomousSystemErrorEntry>,
}
