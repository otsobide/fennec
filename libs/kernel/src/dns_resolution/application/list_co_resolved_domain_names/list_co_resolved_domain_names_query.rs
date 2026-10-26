//! Query for the domain names co-resolved with a domain name.

use shared_cqrs::query::domain::query::Query;

use crate::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;

/// Query that requests the domain names that resolved to one of the
/// addresses `subject` resolved to, at overlapping times.
pub struct ListCoResolvedDomainNamesQuery {
    pub subject: DnsResolutionSubject,
}

impl Query for ListCoResolvedDomainNamesQuery {}
