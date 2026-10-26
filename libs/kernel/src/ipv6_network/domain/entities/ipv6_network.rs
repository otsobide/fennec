//! `Ipv6Network` aggregate root.
//!
//! Represents one IPv6 network that Fennec has registered as an
//! observable. Its identity is its value: two registrations of the same
//! network are the same aggregate.

use crate::ipv6_network::domain::value_objects::ipv6_network_created_at::Ipv6NetworkCreatedAt;
use crate::ipv6_network::domain::value_objects::ipv6_network_id::Ipv6NetworkId;
use crate::ipv6_network::domain::value_objects::ipv6_network_value::Ipv6NetworkValue;

/// Aggregate root modelling an observed IPv6 network.
///
/// The id is always derived from the value, so an `Ipv6Network` whose id does
/// not match its network cannot be built.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Ipv6Network {
    id: Ipv6NetworkId,
    value: Ipv6NetworkValue,
    created_at: Ipv6NetworkCreatedAt,
}

impl Ipv6Network {
    /// Creates an `Ipv6Network` from its value, deriving its id.
    pub fn new(value: Ipv6NetworkValue, created_at: Ipv6NetworkCreatedAt) -> Self {
        Self {
            id: Ipv6NetworkId::from_value(&value),
            value,
            created_at,
        }
    }

    pub fn id(&self) -> &Ipv6NetworkId {
        &self.id
    }

    pub fn value(&self) -> &Ipv6NetworkValue {
        &self.value
    }

    pub fn created_at(&self) -> &Ipv6NetworkCreatedAt {
        &self.created_at
    }
}
