//! Factory function for [`DnsResolutionDeletedEvent`].

use crate::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use crate::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use crate::dns_resolution::domain::events::dns_resolution_deleted_event::DnsResolutionDeletedEvent;

/// Creates a [`DnsResolutionDeletedEvent`] from the deleted resolution.
pub fn create_dns_resolution_deleted_event(
    dns_resolution: &DnsResolution,
) -> Result<DnsResolutionDeletedEvent, DnsResolutionRepositoryError> {
    Ok(DnsResolutionDeletedEvent::new(dns_resolution))
}
