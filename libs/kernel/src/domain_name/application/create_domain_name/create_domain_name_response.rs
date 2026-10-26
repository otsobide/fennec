//! Response for the create-domain-name use case.

use crate::domain_name::application::find_domain_name::find_domain_name_response::DomainNameErrorEntry;

/// Response envelope returned by [`CreateDomainNameCommandHandler`].
///
/// [`CreateDomainNameCommandHandler`]: super::create_domain_name_command_handler::CreateDomainNameCommandHandler
pub struct CreateDomainNameResponse {
    pub error: Option<DomainNameErrorEntry>,
}
