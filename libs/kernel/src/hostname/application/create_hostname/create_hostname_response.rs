//! Response for the create-hostname use case.

use crate::hostname::application::find_hostname::find_hostname_response::HostnameErrorEntry;

/// Response envelope returned by [`CreateHostnameCommandHandler`].
///
/// [`CreateHostnameCommandHandler`]: super::create_hostname_command_handler::CreateHostnameCommandHandler
pub struct CreateHostnameResponse {
    pub error: Option<HostnameErrorEntry>,
}
