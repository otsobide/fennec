//! Factory function for [`DnsResolutionRetractedEvent`].

use crate::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use crate::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use crate::dns_resolution::domain::events::dns_resolution_retracted_event::DnsResolutionRetractedEvent;
use crate::dns_resolution::domain::value_objects::dns_resolution_source_id::DnsResolutionSourceId;

/// Creates a [`DnsResolutionRetractedEvent`] from the resolution and the source that withdrew its observation.
pub fn create_dns_resolution_retracted_event(
    dns_resolution: &DnsResolution,
    source_id: &DnsResolutionSourceId,
) -> Result<DnsResolutionRetractedEvent, DnsResolutionRepositoryError> {
    Ok(DnsResolutionRetractedEvent::new(dns_resolution, source_id))
}
