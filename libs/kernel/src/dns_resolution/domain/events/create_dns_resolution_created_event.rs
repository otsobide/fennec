//! Factory function for [`DnsResolutionCreatedEvent`].

use crate::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use crate::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use crate::dns_resolution::domain::events::dns_resolution_created_event::DnsResolutionCreatedEvent;

/// Creates a [`DnsResolutionCreatedEvent`] from the resolution recorded for the first time.
pub fn create_dns_resolution_created_event(
    dns_resolution: &DnsResolution,
) -> Result<DnsResolutionCreatedEvent, DnsResolutionRepositoryError> {
    Ok(DnsResolutionCreatedEvent::new(dns_resolution))
}
