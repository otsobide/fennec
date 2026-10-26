//! `DomainName` aggregate root.
//!
//! Represents one domain name that Fennec has registered as an
//! observable. Its identity is its value: two registrations of the same
//! name are the same aggregate.

use crate::domain_name::domain::value_objects::domain_name_created_at::DomainNameCreatedAt;
use crate::domain_name::domain::value_objects::domain_name_id::DomainNameId;
use crate::domain_name::domain::value_objects::domain_name_value::DomainNameValue;

/// Aggregate root modelling an observed domain name.
///
/// The id is always derived from the value, so a `DomainName` whose id does
/// not match its name cannot be built.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DomainName {
    id: DomainNameId,
    value: DomainNameValue,
    created_at: DomainNameCreatedAt,
}

impl DomainName {
    /// Creates a `DomainName` from its value, deriving its id.
    pub fn new(value: DomainNameValue, created_at: DomainNameCreatedAt) -> Self {
        Self {
            id: DomainNameId::from_value(&value),
            value,
            created_at,
        }
    }

    pub fn id(&self) -> &DomainNameId {
        &self.id
    }

    pub fn value(&self) -> &DomainNameValue {
        &self.value
    }

    pub fn created_at(&self) -> &DomainNameCreatedAt {
        &self.created_at
    }
}
