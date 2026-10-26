//! Command for withdrawing a source's observation of a DNS resolution.

use shared_cqrs::command::domain::command::Command;

use crate::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;
use crate::dns_resolution::domain::value_objects::dns_resolution_source_id::DnsResolutionSourceId;

/// Command that removes the observation `source_id` made of the resolution
/// `id`. Retracting the last observation deletes the resolution.
pub struct RetractDnsResolutionCommand {
    pub id: DnsResolutionId,
    pub source_id: DnsResolutionSourceId,
}

impl Command for RetractDnsResolutionCommand {}
