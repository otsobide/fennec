//! Factory function for [`DnsResolutionObservedEvent`].

use crate::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use crate::dns_resolution::domain::entities::dns_resolution_observation::DnsResolutionObservation;
use crate::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use crate::dns_resolution::domain::events::dns_resolution_observed_event::DnsResolutionObservedEvent;

/// Creates a [`DnsResolutionObservedEvent`] from the resolution and the observation as it now stands.
pub fn create_dns_resolution_observed_event(
    dns_resolution: &DnsResolution,
    observation: &DnsResolutionObservation,
) -> Result<DnsResolutionObservedEvent, DnsResolutionRepositoryError> {
    Ok(DnsResolutionObservedEvent::new(dns_resolution, observation))
}
