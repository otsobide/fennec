//! `Hostname` aggregate root.
//!
//! Represents one hostname that Fennec has registered as an
//! observable. Its identity is its value: two registrations of the same
//! hostname are the same aggregate.

use crate::hostname::domain::value_objects::hostname_created_at::HostnameCreatedAt;
use crate::hostname::domain::value_objects::hostname_id::HostnameId;
use crate::hostname::domain::value_objects::hostname_value::HostnameValue;

/// Aggregate root modelling an observed hostname.
///
/// The id is always derived from the value, so a `Hostname` whose id does
/// not match its hostname cannot be built.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Hostname {
    id: HostnameId,
    value: HostnameValue,
    created_at: HostnameCreatedAt,
}

impl Hostname {
    /// Creates a `Hostname` from its value, deriving its id.
    pub fn new(value: HostnameValue, created_at: HostnameCreatedAt) -> Self {
        Self {
            id: HostnameId::from_value(&value),
            value,
            created_at,
        }
    }

    pub fn id(&self) -> &HostnameId {
        &self.id
    }

    pub fn value(&self) -> &HostnameValue {
        &self.value
    }

    pub fn created_at(&self) -> &HostnameCreatedAt {
        &self.created_at
    }
}
