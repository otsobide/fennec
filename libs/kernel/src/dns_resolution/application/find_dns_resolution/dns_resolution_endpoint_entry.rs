//! Data entry DTO for an endpoint of a DNS resolution, shared by every use
//! case that returns one.

use crate::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;
use crate::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;

/// Data entry DTO for an endpoint of a resolution, its subject or its
/// answer: its module kind, its canonical value and the id it has in its
/// module.
pub struct DnsResolutionEndpointEntry {
    pub kind: String,
    pub value: String,
    pub id: String,
}

impl DnsResolutionEndpointEntry {
    /// Maps a [`DnsResolutionSubject`] to its entry.
    pub fn from_subject(subject: &DnsResolutionSubject) -> Self {
        Self {
            kind: subject.kind().to_string(),
            value: subject.value().to_string(),
            id: subject.id().to_string(),
        }
    }

    /// Maps a [`DnsResolutionAnswer`] to its entry.
    pub fn from_answer(answer: &DnsResolutionAnswer) -> Self {
        Self {
            kind: answer.kind().to_string(),
            value: answer.to_string(),
            id: answer.id().to_string(),
        }
    }
}
