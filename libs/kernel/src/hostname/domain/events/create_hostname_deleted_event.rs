//! Factory function for [`HostnameDeletedEvent`].

use crate::hostname::domain::entities::hostname::Hostname;
use crate::hostname::domain::errors::hostname_repository_error::HostnameRepositoryError;
use crate::hostname::domain::events::hostname_deleted_event::HostnameDeletedEvent;

/// Creates a [`HostnameDeletedEvent`] from the deleted hostname.
pub fn create_hostname_deleted_event(
    hostname: &Hostname,
) -> Result<HostnameDeletedEvent, HostnameRepositoryError> {
    Ok(HostnameDeletedEvent::new(hostname))
}
