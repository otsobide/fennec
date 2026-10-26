//! Response for the delete-domain-name use case.

use crate::domain_name::application::find_domain_name::find_domain_name_response::DomainNameErrorEntry;

/// Response envelope returned by [`DeleteDomainNameCommandHandler`].
///
/// [`DeleteDomainNameCommandHandler`]: super::delete_domain_name_command_handler::DeleteDomainNameCommandHandler
pub struct DeleteDomainNameResponse {
    pub error: Option<DomainNameErrorEntry>,
}
