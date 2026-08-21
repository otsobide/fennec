//! Response for the create-sighting use case.

use crate::sighting::application::find_sighting::find_sighting_response::SightingErrorEntry;

/// Response envelope returned by [`CreateSightingCommandHandler`].
///
/// [`CreateSightingCommandHandler`]: super::create_sighting_command_handler::CreateSightingCommandHandler
pub struct CreateSightingResponse {
    pub error: Option<SightingErrorEntry>,
}
