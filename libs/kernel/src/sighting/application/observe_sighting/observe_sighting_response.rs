//! Response for the observe-sighting use case.

use crate::sighting::application::find_sighting::find_sighting_response::SightingErrorEntry;

/// Response envelope returned by [`ObserveSightingCommandHandler`].
///
/// [`ObserveSightingCommandHandler`]: super::observe_sighting_command_handler::ObserveSightingCommandHandler
pub struct ObserveSightingResponse {
    pub error: Option<SightingErrorEntry>,
}
