//! Command for registering a domain name.

use shared_cqrs::command::domain::command::Command;

use crate::domain_name::domain::value_objects::domain_name_value::DomainNameValue;

/// Command that requests the registration of a domain name.
///
/// It carries no id: the id of a domain name is derived from its value by
/// [`DomainNameCreator`].
///
/// [`DomainNameCreator`]: super::domain_name_creator::DomainNameCreator
pub struct CreateDomainNameCommand {
    pub value: DomainNameValue,
}

impl Command for CreateDomainNameCommand {}
