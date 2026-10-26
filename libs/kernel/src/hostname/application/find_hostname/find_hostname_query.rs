//! Query for finding a single hostname.

use shared_cqrs::query::domain::query::Query;

use crate::hostname::domain::value_objects::hostname_id::HostnameId;

/// Query that requests the hostname with the given id.
///
/// A lookup by value builds the id from the value first, so it is the same
/// query.
pub struct FindHostnameQuery {
    pub id: HostnameId,
}

impl Query for FindHostnameQuery {}
