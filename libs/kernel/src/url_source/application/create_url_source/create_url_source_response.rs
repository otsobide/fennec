//! Response for the create-url-source use case.

use crate::url_source::application::find_url_source::find_url_source_response::UrlSourceErrorEntry;

/// Response envelope returned by [`CreateUrlSourceCommandHandler`].
///
/// [`CreateUrlSourceCommandHandler`]: super::create_url_source_command_handler::CreateUrlSourceCommandHandler
pub struct CreateUrlSourceResponse {
    pub error: Option<UrlSourceErrorEntry>,
}
