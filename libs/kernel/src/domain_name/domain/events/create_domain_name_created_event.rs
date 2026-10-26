//! Factory function for [`DomainNameCreatedEvent`].

use crate::domain_name::domain::entities::domain_name::DomainName;
use crate::domain_name::domain::errors::domain_name_repository_error::DomainNameRepositoryError;
use crate::domain_name::domain::events::domain_name_created_event::DomainNameCreatedEvent;

/// Creates a [`DomainNameCreatedEvent`] from the given domain name.
pub fn create_domain_name_created_event(
    domain_name: &DomainName,
) -> Result<DomainNameCreatedEvent, DomainNameRepositoryError> {
    Ok(DomainNameCreatedEvent::new(domain_name))
}
