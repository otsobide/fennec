//! Query for finding a single DNS resolution.

use shared_cqrs::query::domain::query::Query;

use crate::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;

/// Query that requests the DNS resolution with the given [`DnsResolutionId`].
///
/// A lookup by endpoints derives the id from them first
/// ([`DnsResolutionId::from_endpoints`]), so it is the same query.
pub struct FindDnsResolutionQuery {
    pub id: DnsResolutionId,
}

impl Query for FindDnsResolutionQuery {}
