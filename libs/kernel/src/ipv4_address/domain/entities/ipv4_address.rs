//! `Ipv4Address` aggregate root.
//!
//! Represents one IPv4 host address that Fennec has registered as an
//! observable. Its identity is its value: two registrations of the same
//! address are the same aggregate.

use crate::ipv4_address::domain::value_objects::ipv4_address_created_at::Ipv4AddressCreatedAt;
use crate::ipv4_address::domain::value_objects::ipv4_address_id::Ipv4AddressId;
use crate::ipv4_address::domain::value_objects::ipv4_address_value::Ipv4AddressValue;

/// Aggregate root modelling an observed IPv4 host address.
///
/// The id is always derived from the value, so an `Ipv4Address` whose id does
/// not match its address cannot be built.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Ipv4Address {
    id: Ipv4AddressId,
    value: Ipv4AddressValue,
    created_at: Ipv4AddressCreatedAt,
}

impl Ipv4Address {
    /// Creates an `Ipv4Address` from its value, deriving its id.
    pub fn new(value: Ipv4AddressValue, created_at: Ipv4AddressCreatedAt) -> Self {
        Self {
            id: Ipv4AddressId::from_value(&value),
            value,
            created_at,
        }
    }

    pub fn id(&self) -> &Ipv4AddressId {
        &self.id
    }

    pub fn value(&self) -> &Ipv4AddressValue {
        &self.value
    }

    pub fn created_at(&self) -> &Ipv4AddressCreatedAt {
        &self.created_at
    }
}
