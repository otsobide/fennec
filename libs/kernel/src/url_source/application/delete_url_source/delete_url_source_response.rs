//! Response for the delete-url-source use case.

use crate::url_source::application::find_url_source::find_url_source_response::UrlSourceErrorEntry;

/// Response envelope returned by [`DeleteUrlSourceCommandHandler`].
///
/// [`DeleteUrlSourceCommandHandler`]: super::delete_url_source_command_handler::DeleteUrlSourceCommandHandler
pub struct DeleteUrlSourceResponse {
    pub error: Option<UrlSourceErrorEntry>,
}
