//! Query for finding a single IPv4 network.

use shared_cqrs::query::domain::query::Query;

use crate::ipv4_network::domain::value_objects::ipv4_network_id::Ipv4NetworkId;

/// Query that requests the IPv4 network with the given id.
///
/// A lookup by value builds the id from the value first, so it is the same
/// query.
pub struct FindIpv4NetworkQuery {
    pub id: Ipv4NetworkId,
}

impl Query for FindIpv4NetworkQuery {}
