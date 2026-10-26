//! Factory function for [`DomainNameDeletedEvent`].

use crate::domain_name::domain::entities::domain_name::DomainName;
use crate::domain_name::domain::errors::domain_name_repository_error::DomainNameRepositoryError;
use crate::domain_name::domain::events::domain_name_deleted_event::DomainNameDeletedEvent;

/// Creates an [`DomainNameDeletedEvent`] from the deleted domain name.
pub fn create_domain_name_deleted_event(
    domain_name: &DomainName,
) -> Result<DomainNameDeletedEvent, DomainNameRepositoryError> {
    Ok(DomainNameDeletedEvent::new(domain_name))
}
