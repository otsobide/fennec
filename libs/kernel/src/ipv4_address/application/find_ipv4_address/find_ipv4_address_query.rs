//! Query for finding a single ipv4 address.

use shared_cqrs::query::domain::query::Query;

use crate::ipv4_address::domain::value_objects::ipv4_address_id::Ipv4AddressId;

/// Query that requests the ipv4 address with the given id.
///
/// A lookup by value builds the id from the value first, so it is the same
/// query.
pub struct FindIpv4AddressQuery {
    pub id: Ipv4AddressId,
}

impl Query for FindIpv4AddressQuery {}
