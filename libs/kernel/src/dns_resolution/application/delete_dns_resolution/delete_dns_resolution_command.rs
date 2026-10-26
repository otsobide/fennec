//! Command for deleting a DNS resolution.

use shared_cqrs::command::domain::command::Command;

use crate::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;

/// Command that requests the deletion of a DNS resolution, with every
/// source's observation of it, by id.
pub struct DeleteDnsResolutionCommand {
    pub id: DnsResolutionId,
}

impl Command for DeleteDnsResolutionCommand {}
