//! Query for finding a single email address.

use shared_cqrs::query::domain::query::Query;

use crate::email_address::domain::value_objects::email_address_id::EmailAddressId;

/// Query that requests the email address with the given id.
///
/// A lookup by value builds the id from the value first, so it is the same
/// query.
pub struct FindEmailAddressQuery {
    pub id: EmailAddressId,
}

impl Query for FindEmailAddressQuery {}
