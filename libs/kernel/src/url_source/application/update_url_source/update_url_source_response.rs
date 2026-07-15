//! Response for the update-url-source use case.

use crate::url_source::application::find_url_source::find_url_source_response::UrlSourceErrorEntry;

/// Response envelope returned by [`UpdateUrlSourceCommandHandler`].
///
/// [`UpdateUrlSourceCommandHandler`]: super::update_url_source_command_handler::UpdateUrlSourceCommandHandler
pub struct UpdateUrlSourceResponse {
    pub error: Option<UrlSourceErrorEntry>,
}
