//! `Ipv4Network` aggregate root.
//!
//! Represents one IPv4 network that Fennec has registered as an
//! observable. Its identity is its value: two registrations of the same
//! network are the same aggregate.

use crate::ipv4_network::domain::value_objects::ipv4_network_created_at::Ipv4NetworkCreatedAt;
use crate::ipv4_network::domain::value_objects::ipv4_network_id::Ipv4NetworkId;
use crate::ipv4_network::domain::value_objects::ipv4_network_value::Ipv4NetworkValue;

/// Aggregate root modelling an observed IPv4 network.
///
/// The id is always derived from the value, so an `Ipv4Network` whose id does
/// not match its network cannot be built.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Ipv4Network {
    id: Ipv4NetworkId,
    value: Ipv4NetworkValue,
    created_at: Ipv4NetworkCreatedAt,
}

impl Ipv4Network {
    /// Creates an `Ipv4Network` from its value, deriving its id.
    pub fn new(value: Ipv4NetworkValue, created_at: Ipv4NetworkCreatedAt) -> Self {
        Self {
            id: Ipv4NetworkId::from_value(&value),
            value,
            created_at,
        }
    }

    pub fn id(&self) -> &Ipv4NetworkId {
        &self.id
    }

    pub fn value(&self) -> &Ipv4NetworkValue {
        &self.value
    }

    pub fn created_at(&self) -> &Ipv4NetworkCreatedAt {
        &self.created_at
    }
}
