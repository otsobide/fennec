//! Command for recording a source's report of a DNS resolution.

use shared_cqrs::command::domain::command::Command;

use crate::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;
use crate::dns_resolution::domain::value_objects::dns_resolution_count::DnsResolutionCount;
use crate::dns_resolution::domain::value_objects::dns_resolution_source_id::DnsResolutionSourceId;
use crate::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;
use crate::dns_resolution::domain::value_objects::dns_resolution_window::DnsResolutionWindow;

/// Command that reports that `source_id` saw `subject` resolve to `answer`
/// within `window`, `count` times if the source counts.
///
/// It carries no id: the id is derived from the endpoints by
/// [`DnsResolutionRecorder`], which creates the resolution or merges the
/// report into it.
///
/// [`DnsResolutionRecorder`]: super::dns_resolution_recorder::DnsResolutionRecorder
pub struct RecordDnsResolutionCommand {
    pub subject: DnsResolutionSubject,
    pub answer: DnsResolutionAnswer,
    pub source_id: DnsResolutionSourceId,
    pub window: DnsResolutionWindow,
    pub count: Option<DnsResolutionCount>,
}

impl Command for RecordDnsResolutionCommand {}
