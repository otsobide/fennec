//! Command for deleting a domain name.

use shared_cqrs::command::domain::command::Command;

use crate::domain_name::domain::value_objects::domain_name_id::DomainNameId;

/// Command that requests the deletion of the domain name with the given id.
pub struct DeleteDomainNameCommand {
    pub id: DomainNameId,
}

impl Command for DeleteDomainNameCommand {}
