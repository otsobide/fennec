//! Response for the delete-sighting use case.

use crate::sighting::application::find_sighting::find_sighting_response::SightingErrorEntry;

/// Response envelope returned by [`DeleteSightingCommandHandler`].
///
/// [`DeleteSightingCommandHandler`]: super::delete_sighting_command_handler::DeleteSightingCommandHandler
pub struct DeleteSightingResponse {
    pub error: Option<SightingErrorEntry>,
}
