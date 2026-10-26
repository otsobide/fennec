//! Query for listing the DNS resolutions to an address.

use shared_cqrs::query::domain::query::Query;

use crate::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;

/// Query that requests the DNS resolutions to the given address, at most
/// [`MAX_LISTED`] of them.
///
/// [`MAX_LISTED`]: super::dns_resolutions_by_address_lister::MAX_LISTED
pub struct ListDnsResolutionsByAddressQuery {
    pub answer: DnsResolutionAnswer,
}

impl Query for ListDnsResolutionsByAddressQuery {}
