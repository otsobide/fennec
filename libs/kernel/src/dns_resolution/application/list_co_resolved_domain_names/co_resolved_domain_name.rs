//! Read model for one domain name found by the co-resolution pivot.

use crate::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;
use crate::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;

/// A domain name that resolved to at least one of the addresses the queried
/// name resolved to, in a window that overlaps the queried name's.
///
/// Invariant: `shared_answers` is sorted by address and holds each address
/// once.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoResolvedDomainName {
    domain_name: DnsResolutionSubject,
    shared_answers: Vec<DnsResolutionAnswer>,
}

impl CoResolvedDomainName {
    /// Builds the entry, sorting the shared answers by address and dropping
    /// repeated ones.
    pub fn new(
        domain_name: DnsResolutionSubject,
        mut shared_answers: Vec<DnsResolutionAnswer>,
    ) -> Self {
        shared_answers.sort_by_key(DnsResolutionAnswer::ip);
        shared_answers.dedup();
        Self {
            domain_name,
            shared_answers,
        }
    }

    /// Returns the co-resolved domain name.
    pub fn domain_name(&self) -> &DnsResolutionSubject {
        &self.domain_name
    }

    /// Returns the addresses both names resolved to, sorted by address.
    pub fn shared_answers(&self) -> &[DnsResolutionAnswer] {
        &self.shared_answers
    }
}
