//! Response for the delete-ioc use case.

use crate::ioc::application::find_ioc::find_ioc_response::IocErrorEntry;

/// Response envelope returned by [`DeleteIocCommandHandler`].
///
/// [`DeleteIocCommandHandler`]: super::delete_ioc_command_handler::DeleteIocCommandHandler
pub struct DeleteIocResponse {
    pub error: Option<IocErrorEntry>,
}
