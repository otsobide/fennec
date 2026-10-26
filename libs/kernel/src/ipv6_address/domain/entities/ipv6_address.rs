//! `Ipv6Address` aggregate root.
//!
//! Represents one IPv6 address that Fennec has registered as an
//! observable. Its identity is its value: two registrations of the same
//! address are the same aggregate.

use crate::ipv6_address::domain::value_objects::ipv6_address_created_at::Ipv6AddressCreatedAt;
use crate::ipv6_address::domain::value_objects::ipv6_address_id::Ipv6AddressId;
use crate::ipv6_address::domain::value_objects::ipv6_address_value::Ipv6AddressValue;

/// Aggregate root modelling an observed IPv6 address.
///
/// The id is always derived from the value, so an `Ipv6Address` whose id does
/// not match its address cannot be built.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Ipv6Address {
    id: Ipv6AddressId,
    value: Ipv6AddressValue,
    created_at: Ipv6AddressCreatedAt,
}

impl Ipv6Address {
    /// Creates an `Ipv6Address` from its value, deriving its id.
    pub fn new(value: Ipv6AddressValue, created_at: Ipv6AddressCreatedAt) -> Self {
        Self {
            id: Ipv6AddressId::from_value(&value),
            value,
            created_at,
        }
    }

    pub fn id(&self) -> &Ipv6AddressId {
        &self.id
    }

    pub fn value(&self) -> &Ipv6AddressValue {
        &self.value
    }

    pub fn created_at(&self) -> &Ipv6AddressCreatedAt {
        &self.created_at
    }
}
