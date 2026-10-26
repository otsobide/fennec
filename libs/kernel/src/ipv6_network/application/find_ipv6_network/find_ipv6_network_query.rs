//! Query for finding a single IPv6 network.

use shared_cqrs::query::domain::query::Query;

use crate::ipv6_network::domain::value_objects::ipv6_network_id::Ipv6NetworkId;

/// Query that requests the IPv6 network with the given id.
///
/// A lookup by value builds the id from the value first, so it is the same
/// query.
pub struct FindIpv6NetworkQuery {
    pub id: Ipv6NetworkId,
}

impl Query for FindIpv6NetworkQuery {}
