//! Outcome of recording a source's report of a DNS resolution.

use crate::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;

/// What [`DnsResolutionRecorder`] did with a report, carrying the id of the
/// resolution in every case.
///
/// [`DnsResolutionRecorder`]: super::dns_resolution_recorder::DnsResolutionRecorder
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DnsResolutionRecording {
    /// The resolution was not recorded yet; it now exists with the report as
    /// its first observation.
    Created(DnsResolutionId),
    /// The resolution existed and the report added an observation or widened
    /// the source's existing one.
    Observed(DnsResolutionId),
    /// The report added nothing to the source's observation: nothing was
    /// written or published.
    Unchanged(DnsResolutionId),
}

impl DnsResolutionRecording {
    /// Returns the id of the resolution the report was about.
    pub fn id(&self) -> &DnsResolutionId {
        match self {
            Self::Created(id) | Self::Observed(id) | Self::Unchanged(id) => id,
        }
    }

    /// Returns the outcome as the HTTP API spells it: `created`, `observed`
    /// or `unchanged`.
    pub fn outcome(&self) -> &'static str {
        match self {
            Self::Created(_) => "created",
            Self::Observed(_) => "observed",
            Self::Unchanged(_) => "unchanged",
        }
    }
}
