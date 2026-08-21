//! Response for the create-ioc use case.

use crate::ioc::application::find_ioc::find_ioc_response::IocErrorEntry;

/// Response envelope returned by [`CreateIocCommandHandler`].
///
/// [`CreateIocCommandHandler`]: super::create_ioc_command_handler::CreateIocCommandHandler
pub struct CreateIocResponse {
    pub error: Option<IocErrorEntry>,
}
