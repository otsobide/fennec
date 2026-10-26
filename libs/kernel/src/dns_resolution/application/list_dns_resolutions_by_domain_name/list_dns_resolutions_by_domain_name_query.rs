//! Query for listing the DNS resolutions of a domain name.

use shared_cqrs::query::domain::query::Query;

use crate::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;

/// Query that requests the DNS resolutions of the given domain name, at most
/// [`MAX_LISTED`] of them.
///
/// [`MAX_LISTED`]: super::dns_resolutions_by_domain_name_lister::MAX_LISTED
pub struct ListDnsResolutionsByDomainNameQuery {
    pub subject: DnsResolutionSubject,
}

impl Query for ListDnsResolutionsByDomainNameQuery {}
