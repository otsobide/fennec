//! Query for finding a single IPv6 address.

use shared_cqrs::query::domain::query::Query;

use crate::ipv6_address::domain::value_objects::ipv6_address_id::Ipv6AddressId;

/// Query that requests the IPv6 address with the given id.
///
/// A lookup by value builds the id from the value first, so it is the same
/// query.
pub struct FindIpv6AddressQuery {
    pub id: Ipv6AddressId,
}

impl Query for FindIpv6AddressQuery {}
