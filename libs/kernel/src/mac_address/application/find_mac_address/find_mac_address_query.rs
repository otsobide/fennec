//! Query for finding a single MAC address.

use shared_cqrs::query::domain::query::Query;

use crate::mac_address::domain::value_objects::mac_address_id::MacAddressId;

/// Query that requests the MAC address with the given id.
///
/// A lookup by value builds the id from the value first, so it is the same
/// query.
pub struct FindMacAddressQuery {
    pub id: MacAddressId,
}

impl Query for FindMacAddressQuery {}
