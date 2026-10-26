//! Factory function for [`DnsResolutionObservedEvent`].

use crate::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use crate::dns_resolution::domain::entities::dns_resolution_observation::DnsResolutionObservation;
use crate::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use crate::dns_resolution::domain::events::dns_resolution_observed_event::DnsResolutionObservedEvent;
use crate::dns_resolution::domain::value_objects::dns_resolution_count::DnsResolutionCount;
use crate::dns_resolution::domain::value_objects::dns_resolution_window::DnsResolutionWindow;

/// Creates a [`DnsResolutionObservedEvent`] from the resolution, the
/// observation as it now stands, and the report the source sent: its
/// window and its count, `None` when the source does not count.
pub fn create_dns_resolution_observed_event(
    dns_resolution: &DnsResolution,
    observation: &DnsResolutionObservation,
    reported_window: &DnsResolutionWindow,
    reported_count: Option<DnsResolutionCount>,
) -> Result<DnsResolutionObservedEvent, DnsResolutionRepositoryError> {
    Ok(DnsResolutionObservedEvent::new(
        dns_resolution,
        observation,
        reported_window,
        reported_count,
    ))
}
