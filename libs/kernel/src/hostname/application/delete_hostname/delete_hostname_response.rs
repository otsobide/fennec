//! Response for the delete-hostname use case.

use crate::hostname::application::find_hostname::find_hostname_response::HostnameErrorEntry;

/// Response envelope returned by [`DeleteHostnameCommandHandler`].
///
/// [`DeleteHostnameCommandHandler`]: super::delete_hostname_command_handler::DeleteHostnameCommandHandler
pub struct DeleteHostnameResponse {
    pub error: Option<HostnameErrorEntry>,
}
