//! Outcome of retracting one source's observation of a [`DnsResolution`].
//!
//! [`DnsResolution`]: crate::dns_resolution::domain::entities::dns_resolution::DnsResolution

use crate::dns_resolution::domain::entities::dns_resolution::DnsResolution;

/// What retracting one source's observation did to a [`DnsResolution`], as
/// [`DnsResolution::retract`] returns it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DnsResolutionRetraction {
    /// The observation was removed and other sources still assert the
    /// resolution, which is returned without it.
    Retracted(DnsResolution),
    /// It was the last observation, so the resolution no longer exists.
    LastObservationRetracted,
    /// The source had not observed the resolution.
    NotObserved,
}
