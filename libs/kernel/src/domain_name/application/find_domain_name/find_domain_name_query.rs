//! Query for finding a single domain name.

use shared_cqrs::query::domain::query::Query;

use crate::domain_name::domain::value_objects::domain_name_id::DomainNameId;

/// Query that requests the domain name with the given id.
///
/// A lookup by value builds the id from the value first, so it is the same
/// query.
pub struct FindDomainNameQuery {
    pub id: DomainNameId,
}

impl Query for FindDomainNameQuery {}
