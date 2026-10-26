//! `MacAddress` aggregate root.
//!
//! Represents one MAC address that Fennec has registered as an
//! observable. Its identity is its value: two registrations of the same
//! address are the same aggregate.

use crate::mac_address::domain::value_objects::mac_address_created_at::MacAddressCreatedAt;
use crate::mac_address::domain::value_objects::mac_address_id::MacAddressId;
use crate::mac_address::domain::value_objects::mac_address_value::MacAddressValue;

/// Aggregate root modelling an observed MAC address.
///
/// The id is always derived from the value, so a `MacAddress` whose id does
/// not match its address cannot be built.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MacAddress {
    id: MacAddressId,
    value: MacAddressValue,
    created_at: MacAddressCreatedAt,
}

impl MacAddress {
    /// Creates a `MacAddress` from its value, deriving its id.
    pub fn new(value: MacAddressValue, created_at: MacAddressCreatedAt) -> Self {
        Self {
            id: MacAddressId::from_value(&value),
            value,
            created_at,
        }
    }

    pub fn id(&self) -> &MacAddressId {
        &self.id
    }

    pub fn value(&self) -> &MacAddressValue {
        &self.value
    }

    pub fn created_at(&self) -> &MacAddressCreatedAt {
        &self.created_at
    }
}
