//! Factory function for [`HostnameCreatedEvent`].

use crate::hostname::domain::entities::hostname::Hostname;
use crate::hostname::domain::errors::hostname_repository_error::HostnameRepositoryError;
use crate::hostname::domain::events::hostname_created_event::HostnameCreatedEvent;

/// Creates a [`HostnameCreatedEvent`] from the given hostname.
pub fn create_hostname_created_event(
    hostname: &Hostname,
) -> Result<HostnameCreatedEvent, HostnameRepositoryError> {
    Ok(HostnameCreatedEvent::new(hostname))
}
