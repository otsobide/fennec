//! `EmailAddress` aggregate root.
//!
//! Represents one email address that Fennec has registered as an
//! observable. Its identity is its value: two registrations of the same
//! address are the same aggregate.

use crate::email_address::domain::value_objects::email_address_created_at::EmailAddressCreatedAt;
use crate::email_address::domain::value_objects::email_address_id::EmailAddressId;
use crate::email_address::domain::value_objects::email_address_value::EmailAddressValue;

/// Aggregate root modelling an observed email address.
///
/// The id is always derived from the value, so an `EmailAddress` whose id does
/// not match its address cannot be built.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EmailAddress {
    id: EmailAddressId,
    value: EmailAddressValue,
    created_at: EmailAddressCreatedAt,
}

impl EmailAddress {
    /// Creates an `EmailAddress` from its value, deriving its id.
    pub fn new(value: EmailAddressValue, created_at: EmailAddressCreatedAt) -> Self {
        Self {
            id: EmailAddressId::from_value(&value),
            value,
            created_at,
        }
    }

    pub fn id(&self) -> &EmailAddressId {
        &self.id
    }

    pub fn value(&self) -> &EmailAddressValue {
        &self.value
    }

    pub fn created_at(&self) -> &EmailAddressCreatedAt {
        &self.created_at
    }
}
